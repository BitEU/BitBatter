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

// --- Game Configuration ---
// You can change this value to adjust the game length.
int TOTAL_INNINGS = 3;

// --- Constants ---
#define FIELD_COLOR 2 // Green background
#define DIRT_COLOR 6  // Brown/dark yellow background
#define LINE_COLOR 15 // White foreground
#define GRASS_CHAR ' '
#define DIRT_CHAR ' '
#define LINE_CHAR 219 // Solid block character '█'

// Console screen buffer handle
HANDLE hConsole;

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
 * @brief Draws the main baseball field using block characters.
 */
void draw_field() {
    // Clear screen and set base color
    system("cls");
    set_color(FIELD_COLOR);

    // Draw Grass (outfield)
    for (int y = 0; y < 25; y++) {
        for (int x = 0; x < 80; x++) {
            draw_block(x, y, GRASS_CHAR, BACKGROUND_GREEN);
        }
    }

    // Draw diamond-shaped infield dirt (more realistic shape)
    for (int y = 10; y <= 20; y++) {
        for (int x = 10; x < 70; x++) {
            // Calculate distance from center point to create diamond shape
            int center_x = 39, center_y = 15;
            int dx = abs(x - center_x);
            int dy = abs(y - center_y);
            
            // Create diamond infield
            if (dx + dy <= 15 && y >= 10) {
                draw_block(x, y, DIRT_CHAR, BACKGROUND_RED | BACKGROUND_GREEN);
            }
        }
    }
    
    // Draw pitcher's mound (larger and more visible)
    for (int py = 14; py <= 16; py++) {
        for (int px = 37; px <= 41; px++) {
            if ((px-39)*(px-39) + (py-15)*(py-15) <= 4) { // Circular mound
                draw_block(px, py, DIRT_CHAR, BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_INTENSITY);
            }
        }
    }

    // Draw foul lines (more prominent and accurate)
    // First base foul line (from home to first base and beyond)
    for(int i = 0; i <= 15; i++) {
        int x = 39 + (i * 11) / 15; // More gradual slope to first base
        int y = 20 - i;
        if (x < 80 && y >= 0) {
            draw_block(x, y, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
        }
    }
    // Third base foul line (from home to third base and beyond)
    for(int i = 0; i <= 15; i++) {
        int x = 39 - (i * 11) / 15; // More gradual slope to third base
        int y = 20 - i;
        if (x >= 0 && y >= 0) {
            draw_block(x, y, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
        }
    }
    
    // Base paths (connect the bases)
    // Home to first base path
    for(int i = 0; i <= 10; i++) {
        int x = 39 + (i * 11) / 10;
        int y = 20 - (i * 5) / 10;
        draw_block(x, y, '.', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    }
    // First to second base path
    for(int i = 0; i <= 10; i++) {
        int x = 50 - (i * 11) / 10;
        int y = 15 - (i * 5) / 10;
        draw_block(x, y, '.', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    }
    // Second to third base path
    for(int i = 0; i <= 10; i++) {
        int x = 39 - (i * 11) / 10;
        int y = 10 + (i * 5) / 10;
        draw_block(x, y, '.', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    }
    // Third to home base path
    for(int i = 0; i <= 10; i++) {
        int x = 28 + (i * 11) / 10;
        int y = 15 + (i * 5) / 10;
        draw_block(x, y, '.', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    }

    // Draw prominent bases (make them clearly visible and properly positioned)
    // Home Plate (at bottom of diamond)
    draw_block(39, 20, 'H', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED);
    draw_block(38, 19, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED);
    draw_block(40, 19, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED);
    
    // First Base (right side of diamond)
    draw_block(50, 15, '1', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(49, 15, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(50, 14, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(49, 14, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    
    // Second Base (top of diamond)
    draw_block(39, 10, '2', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(38, 10, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(39, 9, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(38, 9, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    
    // Third Base (left side of diamond)
    draw_block(28, 15, '3', FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(29, 15, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(28, 14, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);
    draw_block(29, 14, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE);

    // Draw batter's boxes (more realistic)
    // Left batter's box
    for(int i=0; i<4; i++) draw_block(36, 19+i, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    for(int i=0; i<4; i++) draw_block(37, 19+i, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    // Right batter's box
    for(int i=0; i<4; i++) draw_block(41, 19+i, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    for(int i=0; i<4; i++) draw_block(42, 19+i, LINE_CHAR, FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);
    
    // Draw outfield warning track
    for (int x = 5; x < 75; x++) {
        draw_block(x, 2, DIRT_CHAR, BACKGROUND_RED);
        draw_block(x, 3, DIRT_CHAR, BACKGROUND_RED);
    }
}

/**
 * @brief Updates the scoreboard and game state display.
 */
void update_scoreboard() {
    // Set to a neutral color
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE);

    // Team Scores
    gotoxy(2, 1);
    printf("Terminal League Baseball");
    gotoxy(2, 3);
    printf("TEAM            R");
    gotoxy(2, 4);
    printf("--------------- -");
    gotoxy(2, 5);
    printf("%-15.15s %d", visitor_team_name, score[0]);
    gotoxy(2, 6);
    printf("%-15.15s %d", home_team_name, score[1]);

    // Inning
    gotoxy(60, 3);
    printf("INNING: %s %d", (current_half == 0) ? "Top" : "Bot", current_inning);

    // Game State
    gotoxy(60, 5);
    printf("Outs:   %d", outs);
    gotoxy(60, 6);
    printf("Strikes: %d", strikes);
    gotoxy(60, 7);
    printf("Balls:   %d", balls);

    // Base Runners
    gotoxy(60, 9);
    printf("BASES:");
    set_color(bases[2] ? (FOREGROUND_RED | FOREGROUND_INTENSITY) : (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE));
    gotoxy(65, 10); printf("3rd"); // 3rd
    set_color(bases[1] ? (FOREGROUND_RED | FOREGROUND_INTENSITY) : (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE));
    gotoxy(68, 8); printf("2nd"); // 2nd
    set_color(bases[0] ? (FOREGROUND_RED | FOREGROUND_INTENSITY) : (FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE));
    gotoxy(71, 10); printf("1st"); // 1st
}


/**
 * @brief Displays a message in the center of the screen.
 * @param message The string to display.
 * @param delay_ms Time in milliseconds to display the message.
 */
void show_message(const char* message, int delay_ms) {
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_INTENSITY); // Bright Yellow
    gotoxy(25, 23);
    printf("MSG: %-50.50s", message);
    Sleep(delay_ms);
    // Clear message
    gotoxy(25, 23);
    printf("MSG: %-50.50s", "");
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
        int pitch_y = 15;
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
            draw_block(pitch_x, pitch_y, DIRT_CHAR, BACKGROUND_RED | BACKGROUND_GREEN);
            
            // Redraw pitcher's mound properly
            for (int py = 14; py <= 16; py++) {
                for (int px = 37; px <= 41; px++) {
                    if ((px-39)*(px-39) + (py-15)*(py-15) <= 4) {
                        draw_block(px, py, DIRT_CHAR, BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_INTENSITY);
                    }
                }
            }
            
            // Calculate new position
            pitch_y = 15 + (int)(progress * 5); // Moves from y=15 to y=20
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
        draw_block(pitch_x, pitch_y, DIRT_CHAR, BACKGROUND_RED | BACKGROUND_GREEN);


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
    srand(time(NULL));
    
    // Set console to use UTF-8 for block characters
    SetConsoleOutputCP(CP_UTF8);
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
    set_color(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE | FOREGROUND_INTENSITY);
    gotoxy(35, 12);
    printf("GAME OVER!");
    gotoxy(32, 14);
    if (score[1] > score[0]) {
        printf("%s WIN!", home_team_name);
    } else if (score[0] > score[1]) {
        printf("%s WIN!", visitor_team_name);
    } else {
        printf("IT'S A TIE!");
    }
    
    gotoxy(1, 25);
    set_color(7); // Reset to default white on black
    return 0;
}
