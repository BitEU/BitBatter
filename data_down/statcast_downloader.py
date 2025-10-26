import os
import time
import glob
import csv
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.firefox.options import Options as FirefoxOptions
from selenium.webdriver.firefox.service import Service
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import TimeoutException
from selenium.webdriver.common.action_chains import ActionChains
from webdriver_manager.firefox import GeckoDriverManager

# --- Configuration ---

# All 30 MLB team abbreviations
MLB_TEAMS = [
    'ARI', 'ATL', 'BAL', 'BOS', 'CHC', 'CIN', 'CLE', 'COL', 'CWS', 
    'DET', 'HOU', 'KC', 'LAA', 'LAD', 'MIA', 'MIL', 'MIN', 'NYM', 
    'NYY', 'OAK', 'PHI', 'PIT', 'SD', 'SEA', 'SF', 'STL', 'TB', 
    'TEX', 'TOR', 'WSH'
]

PLAYER_TYPES = ['pitcher', 'batter']
YEAR = 2025 # Based on your URL
BASE_URL = "https://baseballsavant.mlb.com/leaderboard/statcast"

# Create a directory for our downloads
DOWNLOAD_DIR = os.path.abspath('statcast_downloads')
os.makedirs(DOWNLOAD_DIR, exist_ok=True)

# --- End Configuration ---

def setup_driver():
    """Configures the Firefox driver for automatic downloads."""
    print(f"Setting up Firefox driver to download files to: {DOWNLOAD_DIR}")
    options = FirefoxOptions()
    
    # Set download preferences
    options.set_preference("browser.download.folderList", 2)  # Use custom download path
    options.set_preference("browser.download.dir", DOWNLOAD_DIR)
    options.set_preference("browser.download.manager.showWhenStarting", False)
    
    # Tell Firefox to automatically save CSV files without asking
    options.set_preference("browser.helperApps.neverAsk.saveToDisk", "text/csv")
    
    # Optional: Run headless (without opening a visible browser window)
    # options.add_argument("--headless") 
    
    # Install or update the GeckoDriver
    service = Service(GeckoDriverManager().install())
    
    driver = webdriver.Firefox(service=service, options=options)
    return driver

def clear_stale_files(directory):
    """Removes any partial or default-named files from previous runs."""
    # <<< FIX #1: Changed "statcast.csv" to "Exit_Velocity.csv"
    default_file = os.path.join(directory, "Exit_Velocity.csv")
    part_files = glob.glob(os.path.join(directory, "*.part"))
    
    if os.path.exists(default_file):
        print(f"Removing stale file: {default_file}")
        os.remove(default_file)
        
    for f in part_files:
        print(f"Removing stale partial file: {f}")
        os.remove(f)

def fill_empty_cells_with_zeros(directory):
    """Process all CSV files in the directory and replace empty cells with 0."""
    print(f"\n--- Post-processing: Filling empty cells with zeros ---")
    csv_files = glob.glob(os.path.join(directory, "*.csv"))
    
    if not csv_files:
        print("No CSV files found to process.")
        return
    
    files_processed = 0
    cells_filled = 0
    
    for csv_file in csv_files:
        try:
            # Read the CSV file
            with open(csv_file, 'r', newline='', encoding='utf-8') as f:
                reader = csv.reader(f)
                rows = list(reader)
            
            if not rows:
                continue
            
            # Track if this file had any changes
            file_modified = False
            
            # Process each row (skip header row at index 0)
            for row_idx, row in enumerate(rows):
                for col_idx, cell in enumerate(row):
                    # Check if cell is empty (empty string or only whitespace)
                    if cell.strip() == '':
                        rows[row_idx][col_idx] = '0'
                        cells_filled += 1
                        file_modified = True
            
            # Write back to the file only if modified
            if file_modified:
                with open(csv_file, 'w', newline='', encoding='utf-8') as f:
                    writer = csv.writer(f)
                    writer.writerows(rows)
                files_processed += 1
                print(f"Processed: {os.path.basename(csv_file)}")
        
        except Exception as e:
            print(f"Error processing {os.path.basename(csv_file)}: {e}")
    
    print(f"\nPost-processing complete: {files_processed} files modified, {cells_filled} empty cells filled with zeros.")

def main():
    driver = setup_driver()
    # <<< FIX #2: Changed "statcast.csv" to "Exit_Velocity.csv"
    default_filepath = os.path.join(DOWNLOAD_DIR, "Exit_Velocity.csv")

    try:
        for p_type in PLAYER_TYPES:
            for team in MLB_TEAMS:
                
                print(f"\n--- Processing: {p_type.capitalize()} for {team} ({YEAR}) ---")
                
                # Define the target filename
                target_filename = f"{p_type}_{team}_{YEAR}.csv"
                target_filepath = os.path.join(DOWNLOAD_DIR, target_filename)
                
                # 1. Check if we already have this file
                if os.path.exists(target_filepath):
                    print(f"Already downloaded: {target_filename}. Skipping.")
                    continue
                    
                # 2. Clear any old 'Exit_Velocity.csv' or '.part' files
                clear_stale_files(DOWNLOAD_DIR)
                
                # 3. Construct the URL and navigate
                # Using a different sort key just to be safe, but yours is fine
                url_params = f"?type={p_type}&year={YEAR}&position=&team={team}&min=q&sort=4&sortDir=desc"
                url = BASE_URL + url_params
                
                print(f"Navigating to: {url}")
                driver.get(url)
                
                try:
                    # 4. Wait for the data table's "loading" overlay to disappear
                    print("Waiting for data to load (overlay to disappear)...")
                    WebDriverWait(driver, 30).until(
                        EC.invisibility_of_element_located((By.ID, "loading_statcast_leaderboard"))
                    )
                    print("Data loaded.")

                    # 5. Wait for PRESENCE using the correct ID
                    print("Looking for download button's presence (using ID 'btnCSV')...")
                    
                    download_button = WebDriverWait(driver, 10).until(
                        EC.presence_of_element_located((By.ID, "btnCSV"))
                    )
                    
                    print("Button is present. Attempting to click with JS in a retry loop...")
                    
                    click_success = False
                    click_start_time = time.time()
                    
                    # Try for 10 seconds to click the button
                    while time.time() - click_start_time < 10: 
                        try:
                            # Use JavaScript click, it's the most reliable
                            driver.execute_script("arguments[0].click();", download_button)
                            print("Click command executed.")
                            click_success = True
                            break # Exit loop if click succeeded
                        except Exception as e:
                            print(f"...click failed, retrying in 0.5s. Error: {str(e)[:100]}...")
                            time.sleep(0.5) 
                    
                    if not click_success:
                        print("...Failed to click the button after 10 seconds.")
                        raise TimeoutException("Custom click loop timed out.")

                    # 6. Wait for the download to complete
                    print("Waiting for download to complete...")
                    downloaded = False
                    timeout = 30  # 30-second timeout for the download
                    start_time = time.time()
                    
                    while time.time() - start_time < timeout:
                        # Check for the .part file (Firefox's temporary download file)
                        part_file_exists = os.path.exists(default_filepath + ".part")
                        # Check for the final CSV file
                        csv_file_exists = os.path.exists(default_filepath)
                        
                        # Download is complete when the CSV exists AND the .part file is gone
                        if csv_file_exists and not part_file_exists:
                            print("Download complete.")
                            downloaded = True
                            break
                        
                        time.sleep(0.5) # Check every half second
                        
                    if not downloaded:
                        print("...Download timed out. Skipping.")
                        continue
                        
                    # 7. Rename the file
                    print(f"Renaming 'Exit_Velocity.csv' to '{target_filename}'")
                    os.rename(default_filepath, target_filepath)
                    print(f"Success! Saved: {target_filename}")
                    
                except TimeoutException as e:
                    print(f"...Process timed out for {p_type}/{team}: {e}. Skipping.")
                except Exception as e:
                    print(f"...An error occurred for {p_type}/{team}: {e}. Skipping.")
                
                # Be a good citizen and pause briefly between requests
                time.sleep(2) 

    finally:
        print("\nAll tasks complete. Closing browser.")
        driver.quit()
        
        # Post-process all downloaded CSV files to fill empty cells with zeros
        fill_empty_cells_with_zeros(DOWNLOAD_DIR)

if __name__ == "__main__":
    main()