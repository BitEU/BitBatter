use crate::data::{BaseballSavantBatter, MLBDataImporter, MLBTeamData};
use crate::players::Position;
use crate::teams::Team;
use anyhow::Result;

pub struct MLBTestData;

impl MLBTestData {
    // Sample Yankees data based on the Baseball Savant format
    pub fn get_sample_yankees_csv() -> &'static str {
        r#""last_name, first_name","player_id","attempts","avg_hit_angle","anglesweetspotpercent","max_hit_speed","avg_hit_speed","ev50","fbld","gb","max_distance","avg_distance","avg_hr_distance","ev95plus","ev95percent","barrels","brl_percent","brl_pa"
"Judge, Aaron","592450","283","18.5","36","118.1","94.8","106.2","98.5","90","469","201","405","157","55.5","68","24.0","13.7"
"Bellinger, Cody","641355","381","18.6","37","110.2","88.9","99","92.2","85.6","451","190","387","150","39.4","28","7.3","5.8"
"Chisholm Jr., Jazz","665862","224","16","33","110.9","89.2","101.5","94.9","83.1","442","186","387","99","44.2","38","17.0","10.1"
"Volpe, Anthony","683011","328","11.9","31","108.7","86.4","95.8","88.2","75.3","436","178","374","89","27.1","34","10.4","6.8"
"Stanton, Giancarlo","519317","287","22.4","42","114.6","91.3","103.5","96.8","88.2","462","199","418","142","49.5","45","15.7","9.4"
"Torres, Gleyber","650402","295","14.2","34","109.8","87.6","98.2","91.4","79.7","441","183","391","105","35.6","32","10.8","6.8"
"Rizzo, Anthony","519203","201","21.8","39","108.9","88.1","99.5","93.7","81.6","448","194","402","78","38.8","18","9.0","5.7"
"Wells, Austin","669330","198","17.3","35","107.2","86.9","97.4","90.1","78.5","425","180","385","68","34.3","22","11.1","7.0"
"Verdugo, Alex","657077","312","9.8","28","106.4","84.2","94.1","87.3","73.9","418","172","378","72","23.1","15","4.8","3.1""#
    }

    // Sample Dodgers data based on the Baseball Savant format
    pub fn get_sample_dodgers_csv() -> &'static str {
        r#""last_name, first_name","player_id","attempts","avg_hit_angle","anglesweetspotpercent","max_hit_speed","avg_hit_speed","ev50","fbld","gb","max_distance","avg_distance","avg_hr_distance","ev95plus","ev95percent","barrels","brl_percent","brl_pa"
"Ohtani, Shohei","660271","321","13.1","35","117.9","92.4","104.8","97.2","86.1","473","195","412","145","45.2","73","22.7","15.2"
"Betts, Mookie","605141","394","18.2","38","107.2","89.3","100.1","93.8","82.4","442","188","398","147","37.3","19","4.8","3.2"
"Freeman, Freddie","518692","308","15.5","36","114.1","90.7","102.3","95.6","84.7","456","192","404","123","39.9","30","9.7","6.5"
"Smith, Will","669257","267","19.8","41","112.3","89.8","101.7","94.3","83.6","449","191","401","109","40.8","28","10.5","7.0"
"Muncy, Max","571970","289","21.2","44","113.8","88.9","100.8","94.1","82.9","458","196","415","118","40.8","35","12.1","8.1"
"Hernandez, Teoscar","606192","298","16.7","37","111.4","89.1","100.3","93.2","81.8","447","189","403","116","38.9","31","10.4","6.9"
"Edman, Tommy","669242","245","12.3","32","108.9","85.7","96.8","89.4","76.3","432","177","388","83","33.9","21","8.6","5.7"
"Taylor, Chris","621035","198","14.6","33","109.2","86.4","97.9","90.7","78.1","438","181","394","71","35.9","18","9.1","6.1"
"Lux, Gavin","666624","156","11.8","30","105.8","84.1","95.2","88.6","75.4","421","175","382","52","33.3","13","8.3","5.6""#
    }

    pub fn create_yankees_team() -> Result<Team> {
        let csv_data = Self::get_sample_yankees_csv();
        let players = MLBDataImporter::parse_baseball_savant_csv(csv_data)?;
        
        let team_data = MLBTeamData {
            team_name: "New York Yankees".to_string(),
            team_id: "147".to_string(),
            players,
        };

        // Define starting positions for Yankees players
        let lineup_positions = vec![
            (Position::RightField, 0),      // Judge
            (Position::FirstBase, 1),       // Bellinger
            (Position::SecondBase, 2),      // Chisholm Jr.
            (Position::Shortstop, 3),       // Volpe
            (Position::DesignatedHitter, 4), // Stanton
            (Position::ThirdBase, 5),       // Torres
            (Position::FirstBase, 6),       // Rizzo (backup)
            (Position::Catcher, 7),         // Wells
            (Position::LeftField, 8),       // Verdugo
        ];

        MLBDataImporter::create_team_from_savant_data(&team_data, &lineup_positions)
    }

    pub fn create_dodgers_team() -> Result<Team> {
        let csv_data = Self::get_sample_dodgers_csv();
        let players = MLBDataImporter::parse_baseball_savant_csv(csv_data)?;
        
        let team_data = MLBTeamData {
            team_name: "Los Angeles Dodgers".to_string(),
            team_id: "119".to_string(),
            players,
        };

        // Define starting positions for Dodgers players
        let lineup_positions = vec![
            (Position::DesignatedHitter, 0), // Ohtani
            (Position::RightField, 1),       // Betts
            (Position::FirstBase, 2),        // Freeman
            (Position::Catcher, 3),          // Smith
            (Position::ThirdBase, 4),        // Muncy
            (Position::LeftField, 5),        // Hernandez
            (Position::SecondBase, 6),       // Edman
            (Position::CenterField, 7),      // Taylor
            (Position::Shortstop, 8),        // Lux
        ];

        MLBDataImporter::create_team_from_savant_data(&team_data, &lineup_positions)
    }

    // Function to download and parse real data from Baseball Savant URLs
    // Note: This would require adding reqwest dependency for HTTP requests
    pub fn download_team_data(team_id: &str, year: u16) -> Result<String> {
        let url = format!(
            "https://baseballsavant.mlb.com/leaderboard/statcast?type=batter&year={}&position=&team={}&min=q&sort=barrels_per_pa&sortDir=desc&csv=true",
            year, team_id
        );
        
        // For now, return sample data based on team ID
        Ok(match team_id {
            "147" => Self::get_sample_yankees_csv().to_string(),
            "119" => Self::get_sample_dodgers_csv().to_string(),
            _ => return Err(anyhow::anyhow!("Unknown team ID: {}", team_id)),
        })
    }

    pub fn create_mlb_teams() -> Result<(Team, Team)> {
        let yankees = Self::create_yankees_team()?;
        let dodgers = Self::create_dodgers_team()?;
        Ok((yankees, dodgers))
    }

    // Function to analyze Baseball Savant data and show how it maps to our game
    pub fn analyze_player_conversion() -> String {
        let csv_data = Self::get_sample_yankees_csv();
        if let Ok(players) = MLBDataImporter::parse_baseball_savant_csv(csv_data) {
            let mut analysis = String::from("MLB Data Conversion Analysis:\n\n");
            
            for player in players.iter().take(3) {
                analysis.push_str(&format!("Player: {}\n", player.full_name()));
                analysis.push_str(&format!("  Baseball Savant Metrics:\n"));
                analysis.push_str(&format!("    Attempts: {}\n", player.attempts));
                analysis.push_str(&format!("    Avg Hit Angle: {}°\n", player.avg_hit_angle));
                analysis.push_str(&format!("    Sweet Spot %: {}%\n", player.anglesweetspotpercent));
                analysis.push_str(&format!("    Max Exit Velo: {} mph\n", player.max_hit_speed));
                analysis.push_str(&format!("    Avg Exit Velo: {} mph\n", player.avg_hit_speed));
                analysis.push_str(&format!("    Barrels: {}\n", player.barrels));
                analysis.push_str(&format!("    Barrel %: {}%\n", player.brl_percent));
                
                let tendencies = player.to_batter_tendencies();
                analysis.push_str(&format!("  Converted Game Attributes:\n"));
                analysis.push_str(&format!("    Contact Rate: {:.3}\n", tendencies.contact_rate));
                analysis.push_str(&format!("    Power Rating: {:.3}\n", tendencies.power_rating));
                analysis.push_str(&format!("    Speed Rating: {:.3}\n", tendencies.speed_rating));
                analysis.push_str(&format!("    Patience Rating: {:.3}\n", tendencies.patience_rating));
                analysis.push_str("\n");
            }
            
            analysis
        } else {
            "Error parsing CSV data".to_string()
        }
    }
}