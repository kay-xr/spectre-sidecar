pub async fn print_output_warning() {
    let logo = r#"
                                               ████████████
                                             ██            ██
                                          ██               ██
                                        ██                █
                                      ██                  █
               ███████████████████████                     ███████████████
             █                                                            █
          ██                                                             ██
        ██                                                             ██
        █                                                            ██   █████████████
         ███████████                       ██████████████████████████   ██             ██
                    █                   ██                            ██               ██
                    ██                ██                            ██                █
                   █                ██                            ██                  █
                   █              ██      ███████████████████████                      ███████████████
                    ██████████████      ██                                                            █
                                      ██                                                             ██
                                    ██                                                             ██
                                    █                                                            ██
                                     ███████████                      ██████████████████████████
                                                █                   ██
                                                █                 ██
                                               █                ██
                                               █              █
                                                █████████████
    "#;
    println!("{logo}");
    println!("This tool is for reading and sorting VRC game logs, and is meant to run as a sidecar for Spectre. Please do not run it directly.");
}