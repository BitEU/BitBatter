// =================================================================================
// Terminal League Baseball
// A simple, Atari-style baseball game for the Windows Console.
//
// To Compile with MSVC (from a Developer Command Prompt):
// cl terminal_baseball.c
//
// To Run:
// terminal_baseball.exe
// =================================================================================

#include <stdio.h>
#include <stdlib.h>
#include <windows.h>
#include <conio.h>
#include <time.h>
#include <string.h>
#include <signal.h>

// --- Game Configuration ---
// You can change this value to adjust the game length.
int TOTAL_INNINGS = 3;

// --- Constants ---
#define FIELD_COLOR 2 // Green background
#define DIRT_COLOR 6  // Brown/dark yellow background
#define LINE_COLOR 15 // White foreground
#define GRASS_CHAR "░" // Light Shade character '░' (U+2591) - UTF-8 encoded
#define DIRT_CHAR "▓" // Dark Shade character '▓' (U+2593) - UTF-8 encoded
#define LINE_CHAR 219 // Solid block character '█'
#define BACKGROUND_DIRT_ORANGE (BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_INTENSITY) // Bright brown/orange dirt color

// ANSI color codes for true color support
#define ANSI_RESET "\033[0m"
#define ANSI_BG_GREEN "\033[48;2;34;139;34m"      // Forest green background
#define ANSI_BG_DIRT "\033[48;2;184;115;51m"      // Natural clay/dirt orange (#B87333)
#define ANSI_FG_WHITE "\033[38;2;255;255;255m"    // White foreground
#define ANSI_FG_BLUE "\033[48;2;0;100;255m"       // Blue foreground

// Console screen buffer handle
HANDLE hConsole;
HANDLE hInput;

// Store original console modes for restoration
DWORD originalOutputMode;
DWORD originalInputMode;
UINT originalOutputCP;

// Game State Variables
int score[2] = {0, 0}; // 0: Visitor, 1: Home
int current_inning = 1;
int current_half = 0; // 0: Top, 1: Bottom
int outs = 0;
int strikes = 0;
int balls = 0;
int bases[3] = {0, 0, 0}; // 1st, 2nd, 3rd base. 1 if runner is on base.

// Team names (as requested for testing)
const char *visitor_team_name = "New York Yankees";
const char *home_team_name = "Boston Red Sox";

// --- Function Declarations ---
void cleanup_console();
void signal_handler(int signal);
BOOL WINAPI console_handler(DWORD signal);

// --- Console & Drawing Functions ---

/**
 * @brief Moves the console cursor to a specified X, Y position.
 * @param x The column coordinate.
 * @param y The row coordinate.
 */
void gotoxy(int x, int y) {
    COORD coord;
    coord.X = x;
    coord.Y = y;
    SetConsoleCursorPosition(hConsole, coord);
}

/**
 * @brief Sets the foreground and background color of the console text.
 * @param color The color attribute. See Windows Console color attributes.
 */
void set_color(int color) {
    SetConsoleTextAttribute(hConsole, color);
}

/**
 * @brief Hides the blinking cursor for a cleaner look.
 */
void hide_cursor() {
   CONSOLE_CURSOR_INFO info;
   info.dwSize = 100;
   info.bVisible = FALSE;
   SetConsoleCursorInfo(hConsole, &info);
}

/**
 * @brief Shows the cursor again.
 */
void show_cursor() {
   CONSOLE_CURSOR_INFO info;
   info.dwSize = 25;
   info.bVisible = TRUE;
   SetConsoleCursorInfo(hConsole, &info);
}

/**
 * @brief Signal handler for graceful exit.
 */
void signal_handler(int signal) {
    cleanup_console();
    printf("\nProgram interrupted. Console has been restored.\n");
    exit(0);
}

/**
 * @brief Console control handler for Windows console events.
 */
BOOL WINAPI console_handler(DWORD signal) {
    if (signal == CTRL_C_EVENT || signal == CTRL_CLOSE_EVENT || signal == CTRL_BREAK_EVENT) {
        cleanup_console();
        printf("\nProgram interrupted. Console has been restored.\n");
        ExitProcess(0);
        return TRUE;
    }
    return FALSE;
}

/**
 * @brief Cleanup function called at program exit.
 */
void exit_cleanup() {
    if (hConsole != INVALID_HANDLE_VALUE) {
        // Show cursor
        CONSOLE_CURSOR_INFO info;
        info.dwSize = 25;
        info.bVisible = TRUE;
        SetConsoleCursorInfo(hConsole, &info);
        
        // Restore original console modes
        SetConsoleMode(hConsole, originalOutputMode);
        if (hInput != INVALID_HANDLE_VALUE) {
            SetConsoleMode(hInput, originalInputMode);
        }
        
        // Restore original code page
        SetConsoleOutputCP(originalOutputCP);
        
        // Reset to default colors
        SetConsoleTextAttribute(hConsole, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
        
        // Clear screen
        system("cls");
    }
}

/**
 * @brief Resets console to default state and cleans up before exit.
 */
void cleanup_console() {
    // Show cursor first
    show_cursor();
    
    // Restore original console modes
    if (hConsole != INVALID_HANDLE_VALUE) {
        SetConsoleMode(hConsole, originalOutputMode);
    }
    if (hInput != INVALID_HANDLE_VALUE) {
        SetConsoleMode(hInput, originalInputMode);
    }
    
    // Restore original code page
    SetConsoleOutputCP(originalOutputCP);
    
    // Reset to default colors (white text on black background)
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    
    // Clear the screen completely
    system("cls");
    
    // Move cursor to top-left
    gotoxy(0, 0);
    
    // Clear any remaining screen buffer artifacts
    CONSOLE_SCREEN_BUFFER_INFO csbi;
    if (GetConsoleScreenBufferInfo(hConsole, &csbi)) {
        DWORD dwSize = csbi.dwSize.X * csbi.dwSize.Y;
        DWORD dwWritten;
        COORD coord = {0, 0};
        
        // Fill screen with spaces
        FillConsoleOutputCharacter(hConsole, ' ', dwSize, coord, &dwWritten);
        
        // Reset all attributes to default
        FillConsoleOutputAttribute(hConsole, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE, 
                                 dwSize, coord, &dwWritten);
    }
    
    printf("Thanks for playing Terminal League Baseball!\n");
    printf("Press any key to exit...\n");
    _getch();
}


/**
 * @brief Draws a single block of the field with a specified character and color.
 * @param x The column coordinate.
 * @param y The row coordinate.
 * @param ch The character to draw.
 * @param color The color attribute.
 */
void draw_block(int x, int y, char ch, int color) {
    gotoxy(x, y);
    set_color(color);
    printf("%c", ch);
}

/**
 * @brief Draws a single block of the field with a specified string and color.
 * @param x The column coordinate.
 * @param y The row coordinate.
 * @param str The string to draw (for UTF-8 characters).
 * @param color The color attribute.
 */
void draw_block_str(int x, int y, const char* str, int color) {
    gotoxy(x, y);
    set_color(color);
    printf("%s", str);
}

/**
 * @brief Draws a single block using ANSI escape sequences for true color.
 * @param x The column coordinate.
 * @param y The row coordinate.
 * @param str The string to draw.
 * @param ansi_color The ANSI color escape sequence.
 */
void draw_block_ansi(int x, int y, const char* str, const char* ansi_color) {
    gotoxy(x, y);
    printf("%s%s%s", ansi_color, str, ANSI_RESET);
}

/**
 * @brief Draws the main baseball field using block characters.
 */
void draw_field() {
    // Clear screen and set base color
    system("cls");
    set_color(FIELD_COLOR);

    // Draw Grass (outfield) - start lower to make room for UI
    for (int y = 9; y < 25; y++) {
        for (int x = 0; x < 80; x++) {
            draw_block_ansi(x, y, GRASS_CHAR, ANSI_BG_GREEN);
        }
    }

    // Draw diamond-shaped infield dirt (more realistic shape) - adjusted Y coordinates
    for (int y = 15; y <= 22; y++) {
        for (int x = 10; x < 70; x++) {
            // Calculate distance from center point to create diamond shape
            int center_x = 39, center_y = 18;
            int dx = abs(x - center_x);
            int dy = abs(y - center_y);
            
            // Create diamond infield
            if (dx + dy <= 15 && y >= 15) {
                draw_block_ansi(x, y, DIRT_CHAR, ANSI_BG_DIRT);
            }
        }
    }
    
    // Draw pitcher's mound (larger and more visible) - adjusted Y coordinates
    for (int py = 17; py <= 19; py++) {
        for (int px = 37; px <= 41; px++) {
            if ((px-39)*(px-39) + (py-18)*(py-18) <= 4) { // Circular mound
                draw_block_ansi(px, py, DIRT_CHAR, ANSI_BG_DIRT);
            }
        }
    }

    // Draw foul lines (more prominent and accurate) - adjusted coordinates
    // First base foul line (from home to first base and beyond)
    for(int i = 0; i <= 15; i++) {
        int x = 39 + (i * 11) / 15; // More gradual slope to first base
        int y = 23 - i;
        if (x < 80 && y >= 9) {
            draw_block(x, y, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
        }
    }
    // Third base foul line (from home to third base and beyond)
    for(int i = 0; i <= 15; i++) {
        int x = 39 - (i * 11) / 15; // More gradual slope to third base
        int y = 23 - i;
        if (x >= 0 && y >= 9) {
            draw_block(x, y, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
        }
    }
    
    // Base paths (connect the bases) - adjusted coordinates
    // Home to first base path
    for(int i = 0; i <= 10; i++) {
        int x = 39 + (i * 11) / 10;
        int y = 23 - (i * 5) / 10;
        draw_block(x, y, '.', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    }
    // First to second base path
    for(int i = 0; i <= 10; i++) {
        int x = 50 - (i * 11) / 10;
        int y = 18 - (i * 5) / 10;
        draw_block(x, y, '.', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    }
    // Second to third base path
    for(int i = 0; i <= 10; i++) {
        int x = 39 - (i * 11) / 10;
        int y = 13 + (i * 5) / 10;
        draw_block(x, y, '.', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    }
    // Third to home base path
    for(int i = 0; i <= 10; i++) {
        int x = 28 + (i * 11) / 10;
        int y = 18 + (i * 5) / 10;
        draw_block(x, y, '.', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    }

    // Draw prominent bases (make them clearly visible and properly positioned) - adjusted coordinates
    // Home Plate (at bottom of diamond) - White base
    draw_block(39, 23, 'H', FOREGROUND_BLUE | FOREGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(38, 22, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(40, 22, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    
    // First Base (right side of diamond) - White base
    draw_block(50, 18, '1', FOREGROUND_BLUE | FOREGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(49, 18, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(50, 17, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(49, 17, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    
    // Second Base (top of diamond) - White base
    draw_block(39, 13, '2', FOREGROUND_BLUE | FOREGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(38, 13, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(39, 12, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(38, 12, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    
    // Third Base (left side of diamond) - White base
    draw_block(28, 18, '3', FOREGROUND_BLUE | FOREGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(29, 18, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(28, 17, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(29, 17, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);

    // Draw batter's boxes (more realistic) - adjusted coordinates
    // Left batter's box
    for(int i=0; i<4; i++) draw_block(36, 22+i, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    for(int i=0; i<4; i++) draw_block(37, 22+i, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    // Right batter's box
    for(int i=0; i<4; i++) draw_block(41, 22+i, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    for(int i=0; i<4; i++) draw_block(42, 22+i, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    
    // Draw outfield warning track (brown/tan color) - adjusted coordinates
    for (int x = 5; x < 75; x++) {
        draw_block_ansi(x, 9, DIRT_CHAR, ANSI_BG_DIRT);
        draw_block_ansi(x, 10, DIRT_CHAR, ANSI_BG_DIRT);
    }
}

/**
 * @brief Updates the scoreboard and game state display.
 */
void update_scoreboard() {
    // Title with bright colors - positioned higher to avoid field
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY | BACKGROUND_BLUE);
    gotoxy(25, 0);
    printf("   Terminal League Baseball   ");
    
    // Reset color for scoreboard
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);

    // Team Scores with improved formatting - positioned at left edge
    gotoxy(0, 2);
    printf("┌─────────────────────────────┐");
    gotoxy(0, 3);
    printf("│ TEAM                    R   │");
    gotoxy(0, 4);
    printf("├─────────────────────────────┤");
    gotoxy(0, 5);
    printf("│ %-15.15s         %d   │", visitor_team_name, score[0]);
    gotoxy(0, 6);
    printf("│ %-15.15s         %d   │", home_team_name, score[1]);
    gotoxy(0, 7);
    printf("└─────────────────────────────┘");

    // Game status with colored background - positioned to align with other boxes
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY | BACKGROUND_GREEN);
    gotoxy(32, 2);
    printf("  INNING: %s %d  ", (current_half == 0) ? "Top" : "Bot", current_inning);

    // Game State with box drawing - properly aligned
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
    gotoxy(32, 4);
    printf("┌─────────────┐");
    gotoxy(32, 5);
    printf("│ Outs:    %d  │", outs);
    gotoxy(32, 6);
    printf("│ Strikes: %d  │", strikes);
    gotoxy(32, 7);
    printf("│ Balls:   %d  │", balls);
    gotoxy(32, 8);
    printf("└─────────────┘");

    // Base Runners with improved layout - positioned further right
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
    gotoxy(47, 4);
    printf("┌─ BASES ──────┐");
    gotoxy(47, 5);
    printf("│      ");
    set_color(bases[1] ? (FOREGROUND_RED | FOREGROUND_INTENSITY | BACKGROUND_RED) : (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE));
    printf("2nd");
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
    printf("     │");
    gotoxy(47, 6);
    printf("│  ");
    set_color(bases[2] ? (FOREGROUND_RED | FOREGROUND_INTENSITY | BACKGROUND_RED) : (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE));
    printf("3rd");
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
    printf("   ");
    set_color(bases[0] ? (FOREGROUND_RED | FOREGROUND_INTENSITY | BACKGROUND_RED) : (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE));
    printf("1st");
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
    printf("   │");
    gotoxy(47, 7);
    printf("└──────────────┘");
}


/**
 * @brief Displays a message in the center of the screen.
 * @param message The string to display.
 * @param delay_ms Time in milliseconds to display the message.
 */
void show_message(const char* message, int delay_ms) {
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY | BACKGROUND_BLUE); // Bright Yellow on Blue
    gotoxy(10, 25);
    printf("┌─ MESSAGE ─────────────────────────────────────────────────┐");
    gotoxy(10, 26);
    printf("│ %-57.57s│", message);
    gotoxy(10, 27);
    printf("└───────────────────────────────────────────────────────────┘");
    Sleep(delay_ms);
    // Clear message
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    gotoxy(10, 25);
    printf("                                                             ");
    gotoxy(10, 26);
    printf("                                                             ");
    gotoxy(10, 27);
    printf("                                                             ");
}


// --- Game Logic Functions ---

/**
 * @brief Resets the bases after an inning change or scoring play.
 */
void clear_bases() {
    bases[0] = 0;
    bases[1] = 0;
    bases[2] = 0;
}

/**
 * @brief Advances runners based on the type of hit.
 * @param hit_type 1 for single, 2 for double, 3 for triple, 4 for home run.
 */
void advance_runners(int hit_type) {
    for (int i = 0; i < hit_type; i++) {
        // Runner from 3rd scores
        if (bases[2] == 1) {
            score[current_half]++;
            bases[2] = 0;
        }
        // Runner from 2nd to 3rd
        if (bases[1] == 1) {
            bases[2] = 1;
            bases[1] = 0;
        }
        // Runner from 1st to 2nd
        if (bases[0] == 1) {
            bases[1] = 1;
            bases[0] = 0;
        }
    }
    // Place the batter on base
    if (hit_type > 0 && hit_type < 4) {
        bases[hit_type - 1] = 1;
    } else if (hit_type == 4) { // Home run
        score[current_half]++;
    }
}

/**
 * @brief Manages a single at-bat, from pitch to outcome.
 */
void play_at_bat() {
    strikes = 0;
    balls = 0;
    update_scoreboard();

    while (strikes < 3 && balls < 4) {
        show_message("Pitcher is ready... Press SPACE to swing!", 100);

        // --- Pitch Animation ---
        int pitch_land_time = 250 + (rand() % 250); // Pitch will take 250-500ms
        int pitch_y = 18;  // Updated to match new pitcher's mound position
        int pitch_x = 39;
        char ball_char = 'o';
        
        // Pitcher winds up
        gotoxy(pitch_x, pitch_y);
        set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
        printf("P");
        Sleep(500);

        // Animate the pitch
        long long start_time = GetTickCount64();
        long long current_time = start_time;
        int swing_time = -1;

        while(current_time - start_time < pitch_land_time) {
            current_time = GetTickCount64();
            float progress = (float)(current_time - start_time) / pitch_land_time;
            
            // Clear previous ball position
            gotoxy(pitch_x, pitch_y);
            draw_block_ansi(pitch_x, pitch_y, DIRT_CHAR, ANSI_BG_DIRT);
            
            // Redraw pitcher's mound properly - updated coordinates
            for (int py = 17; py <= 19; py++) {
                for (int px = 37; px <= 41; px++) {
                    if ((px-39)*(px-39) + (py-18)*(py-18) <= 4) {
                        draw_block_ansi(px, py, DIRT_CHAR, ANSI_BG_DIRT);
                    }
                }
            }
            
            // Calculate new position - updated to match new field layout
            pitch_y = 18 + (int)(progress * 5); // Moves from y=18 to y=23
            pitch_x = 39;

            // Draw ball
            gotoxy(pitch_x, pitch_y);
            set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
            printf("%c", ball_char);
            
            // Check for player input (the swing)
            if (_kbhit()) {
                if (_getch() == ' ') {
                    if(swing_time == -1) { // Only register first swing
                       swing_time = (int)(current_time - start_time);
                    }
                }
            }
            Sleep(10); // Animation speed
        }
        
        // Clear the ball after it crosses the plate
        draw_block_ansi(pitch_x, pitch_y, DIRT_CHAR, ANSI_BG_DIRT);


        // --- Determine Outcome ---
        int perfect_swing_window_start = (int)(pitch_land_time * 0.75);
        int perfect_swing_window_end = pitch_land_time;

        if (swing_time != -1) { // Player swung
            if (swing_time >= perfect_swing_window_start && swing_time <= perfect_swing_window_end) {
                // HIT!
                int hit_type = (rand() % 100);
                if (hit_type < 5) { // 5% chance of HR
                    show_message("HOME RUN!!!", 2000);
                    advance_runners(4);
                } else if (hit_type < 15) { // 10% chance of triple
                    show_message("TRIPLE! A shot to the gap!", 2000);
                    advance_runners(3);
                } else if (hit_type < 35) { // 20% chance of double
                    show_message("DOUBLE! Down the line!", 2000);
                    advance_runners(2);
                } else { // 65% chance of single
                    show_message("SINGLE! A base hit.", 2000);
                    advance_runners(1);
                }
                return; // End at-bat
            } else {
                // SWING AND A MISS
                strikes++;
                show_message("SWING AND A MISS! Strike!", 1500);
            }
        } else { // Player did not swing
            // For simplicity, we'll make all non-swings a called strike.
            // A real game would have balls/strikes based on location.
            strikes++;
            show_message("Called Strike!", 1500);
        }

        update_scoreboard();

        if (strikes >= 3) {
            outs++;
            show_message("STRIKEOUT!", 2000);
        }
        if (balls >= 4) {
            // This part is not reachable in the current simple logic,
            // but is here for future expansion.
            show_message("WALK! Take your base.", 2000);
            // advance_runners for a walk
            // ...
        }
    }
}

// --- Main Game Controller ---

int main() {
    // --- Initialization ---
    hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    hInput = GetStdHandle(STD_INPUT_HANDLE);
    srand(time(NULL));
    
    // Store original console modes and code page for restoration
    GetConsoleMode(hConsole, &originalOutputMode);
    GetConsoleMode(hInput, &originalInputMode);
    originalOutputCP = GetConsoleOutputCP();
    
    // Register cleanup function to run at exit
    atexit(exit_cleanup);
    
    // Set up signal handlers for graceful exit
    signal(SIGINT, signal_handler);
    signal(SIGTERM, signal_handler);
    SetConsoleCtrlHandler(console_handler, TRUE);
    
    // Set console to use UTF-8 for Unicode characters
    SetConsoleOutputCP(CP_UTF8);
    SetConsoleCP(CP_UTF8); // Also set input code page
    // Needed for block characters to render correctly
    SetConsoleMode(hConsole, ENABLE_VIRTUAL_TERMINAL_PROCESSING);

    hide_cursor();
    
    // --- Game Loop ---
    while (current_inning <= TOTAL_INNINGS) {
        draw_field();
        outs = 0;
        strikes = 0;
        balls = 0;
        clear_bases();
        
        char half_inning_msg[50];
        sprintf(half_inning_msg, "%s of the %d inning.", (current_half == 0) ? "Top" : "Bottom", current_inning);
        update_scoreboard();
        show_message(half_inning_msg, 2000);

        while (outs < 3) {
            play_at_bat();
            update_scoreboard();
            if (outs >= 3) break;
        }

        // Switch half-innings
        if (current_half == 0) {
            current_half = 1;
        } else {
            current_half = 0;
            current_inning++;
        }
        
        // End game condition
        if (current_inning > TOTAL_INNINGS) {
            // Check for tie game after final inning
            if (score[0] == score[1] && current_half == 0) {
                // Go to extra innings
                TOTAL_INNINGS++;
                show_message("TIE GAME! We are going to extra innings!", 2500);
            } else {
                break;
            }
        }
    }

    // --- Game Over ---
    draw_field();
    update_scoreboard();
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY | BACKGROUND_BLUE);
    gotoxy(30, 12);
    printf("   GAME OVER!   ");
    gotoxy(25, 14);
    if (score[1] > score[0]) {
        printf("  %s WIN!  ", home_team_name);
    } else if (score[0] > score[1]) {
        printf("  %s WIN!  ", visitor_team_name);
    } else {
        printf("   IT'S A TIE!   ");
    }
    
    // Wait for user input before cleanup
    gotoxy(25, 16);
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
    printf("Press any key to exit...");
    _getch();
    
    // Clean up and reset console
    cleanup_console();
    return 0;
}
