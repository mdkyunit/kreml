use std::io;
use std::io::{stdout, Write};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    event,
};
use crossterm::terminal::{Clear, ClearType};
pub enum MenuType {
    MainMenu,
    GameMenu,
    SplashMenu,
}
/*
     =======DISPLAY ENGINE=======
     This file contains the functions needed
     to display items such as the main menu,
     user interface and as well as additional information.
     This section is currently being developed and is not functional 
     as of now
 */
pub fn asciidisplay(menu: MenuType) {
    match menu {
        MenuType::MainMenu => {},
        MenuType::GameMenu => {
            game_menu();
        },
        MenuType::SplashMenu => {
            splashscreen();
        },
    }
}
pub fn splashscreen() -> std::io::Result<()> { // For games splash screen when its opened
    stdout()
        .execute(Clear(ClearType::All))?
        .execute(SetForegroundColor(Color::Red))?
        .execute(SetBackgroundColor(Color::Black))?
        .execute(Print(r" █████                                         ████
░░███                                         ░░███
 ░███ █████ ████████   ██████  █████████████   ░███
 ░███░░███ ░░███░░███ ███░░███░░███░░███░░███  ░███
 ░██████░   ░███ ░░░ ░███████  ░███ ░███ ░███  ░███
 ░███░░███  ░███     ░███░░░   ░███ ░███ ░███  ░███
 ████ █████ █████    ░░██████  █████░███ █████ █████
░░░░ ░░░░░ ░░░░░      ░░░░░░  ░░░░░ ░░░ ░░░░░ ░░░░░ "))?
        .execute(ResetColor)?;
    Ok(())

}
pub fn game_menu() -> std::io::Result<()> {
    stdout()
        //.execute(Clear(ClearType::All))?
        .execute(Print(
            ("===================================
            Command Mode
===================================
INPUT: ")
        ))?;
    Ok(())
}