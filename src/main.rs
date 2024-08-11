use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
    rand::gen_range
};

type A = f32;

const MAX_SCORE: u8 = 20;
const PLAYER_LIFE: u8 = 10;
const MAX_BULLET: u8 = 2;
const MAX_ENEMY: u8 = 10;
const SCREEN_WIDTH: f32 = 800.;
const SCREEN_HEIGHT: f32 = 600.;

enum GameState {
    Menu,
    Credit,
    InGame,
    EndGame
}

#[derive(Default)]
struct Player {
    pos: Vec2,
    size: Vec2,
    speed: f32,
    health: u8
}

#[derive(Default)]
struct Enemy {
    pos: Vec2,
    size: Vec2,
    active: bool
}

#[derive(Default)]
struct SUPERULTRADUPERMEGACOOLSHOOTINGOBJECTFORABSOLUTELYNOREASONATALL {
    pos: Vec2,
    size: Vec2,
    health: u8,
    speed: f32
}

#[derive(Default)]
struct Bullet {
    pos: Vec2,
    size: Vec2,
    speed: f32,
    active: bool
}

struct Game {
    gamestate: GameState,
    player: Player,
    bullet: Vec<Bullet>,
    enemy: Vec<Enemy>,
    superultradupermegacoolshootingobjectforabsolutelynoreasonatall: SUPERULTRADUPERMEGACOOLSHOOTINGOBJECTFORABSOLUTELYNOREASONATALL,
    score: u8,
    exit: bool
}

//initialize game
impl Default for Game {
    fn default() -> Self {
        let mut bullet = Vec::new();
        let mut enemy = Vec::new();

        for _ in 0..MAX_BULLET {
            bullet.push(Bullet::default());
        }

        for _ in 0..MAX_ENEMY {
            enemy.push(Enemy::default());
        }

        Game {
            gamestate: GameState::Menu,
            player: Player::default(),
            bullet,
            enemy,
            superultradupermegacoolshootingobjectforabsolutelynoreasonatall: SUPERULTRADUPERMEGACOOLSHOOTINGOBJECTFORABSOLUTELYNOREASONATALL::default(),
            score: 0,
            exit: false
        }
    }
}

//initialize window's properties
fn windows_init() -> Conf {
    Conf {
        window_title: "space thingy".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(windows_init)]
async fn main() {
    let mut game = Game::default();

    //initialize buttons' skin
    let skin = {
        let button_style = root_ui()
        .style_builder()
        .text_color(GREEN)
        .font_size(30)
        .build();

        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };

    root_ui().push_skin(&skin);
    
    //main game loop
    loop {
        update(&mut game);
        draw(&game);
        
        if game.exit || is_key_down(KeyCode::Escape) {break;}
        
        next_frame().await;
    }
}


fn init_game(game: &mut Game) {

    //initialize player
    game.player.pos = Vec2::new(SCREEN_WIDTH / 2., SCREEN_HEIGHT - 50.);
    game.player.size = Vec2::new(20., 20.);
    game.player.speed = 7.;
    game.player.health = PLAYER_LIFE;

    //initialize bullets
    for bullets in &mut game.bullet {
        bullets.pos = Vec2::default();
        bullets.size = Vec2::new(5., 15.);
        bullets.speed = 30.;
        bullets.active = false;
    }

    //initialize enemies
    for enemies in &mut game.enemy {

        enemies.pos = Vec2::new(
            gen_range(0., SCREEN_WIDTH),
            gen_range(100., 200.)
        );
        
        enemies.size = Vec2::new(20., 20.);
        enemies.active = true;
    }

    //initialize superultradupermegacoolshootingobjectforabsolutelynoreasonatall
    game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.pos = Vec2::new(SCREEN_WIDTH / 2., 50.);
    game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.health = 10;
    game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.size = Vec2::new(30., 30.);
    game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.speed = 5.;
}

fn update(game: &mut Game) {
    match game.gamestate {
        GameState::Menu => {
            //buttons' position
            let play_pos = Vec2::new(100., 150.);
            let credit_pos = Vec2::new(100., 200.);
            let quit_pos = Vec2::new(100., 250.);
            
            //draw buttons
            if root_ui().button(play_pos, "Play") {init_game(game); game.gamestate = GameState::InGame;}
            if root_ui().button(credit_pos, "Credit") {game.gamestate = GameState::Credit;}
            if root_ui().button(quit_pos, "Quit") {game.exit = true;}
        }

        GameState::Credit => {
            if root_ui().button(vec2(SCREEN_WIDTH / 2., SCREEN_HEIGHT - 200.), "Back to menu") {game.gamestate = GameState::Menu;}
        }

        GameState::InGame => {
            //player's movement
            if (is_key_down(KeyCode::A) || is_key_down(KeyCode::Left)) && !is_key_down(KeyCode::D) && !is_key_down(KeyCode::Right) {
                game.player.pos.x -= game.player.speed;
            } else if (is_key_down(KeyCode::D) || is_key_down(KeyCode::Right)) && !is_key_down(KeyCode::A) && !is_key_down(KeyCode::Left) {
                game.player.pos.x += game.player.speed;
            }

            if is_key_pressed(KeyCode::Space) {
                for bullets in &mut game.bullet {
                    if !bullets.active {
                        bullets.pos = Vec2::new(game.player.pos.x + (game.player.size.x / 2.) - 1., game.player.pos.y);

                        bullets.active = true;

                        break;
                    }
                }
            }

            //player's movement limit
            if game.player.pos.x >= SCREEN_WIDTH {
                game.player.pos.x = 1.;
            } else if game.player.pos.x <= 0. {
                game.player.pos.x = SCREEN_WIDTH - 1.;
            }

            //bullets' properties
            for bullets in &mut game.bullet {
                if bullets.active {
                    bullets.pos.y -= bullets.speed;

                    if bullets.pos.y <= 0. {
                        bullets.pos = Vec2::default();
                        bullets.active = false;
                    }

                    //collision with that thing
                    if game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.health > 0 && game.score >= 10 && collision(&game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.pos.x, &game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.pos.y, &game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.size.x, &game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.size.y, &bullets.pos.x, &bullets.pos.y, &bullets.size.x, &bullets.size.y) {
                        bullets.pos = Vec2::default();
                        bullets.active = false;
                        game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.health -= 1;
                    }

                    //collision with enemies
                    for enemies in &mut game.enemy {
                        if enemies.active && collision(&enemies.pos.x, &enemies.pos.y, &enemies.size.x, &enemies.size.y, &bullets.pos.x, &bullets.pos.y, &bullets.size.x, &bullets.size.y) {
                            bullets.pos = Vec2::default();
                            bullets.active = false;
                            game.score += 1;
                            enemies.active = false;
                        }
                    }
                }
            }

            //that thing's movement
            if game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.health > 0 && game.score >= 10 {
                if game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.pos.x > SCREEN_WIDTH - 20. {game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.speed *= -1.;}
                if game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.pos.x < 0. {game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.speed *= -1.;}

                game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.pos.x += game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.speed;
            } else if game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.health <= 0 {game.score += 10;}

            //winning condition
            if game.score >= MAX_SCORE {
                game.gamestate = GameState::EndGame;
            }
        }

        GameState::EndGame => {
            if root_ui().button(vec2(100., 200.), "Back to menu") {game.score = 0; game.gamestate = GameState::Menu;}
            if root_ui().button(vec2(100., 300.), "Quit") {game.exit = true;}
        }
    }
}

fn draw(game: &Game) {
    match game.gamestate {
        //Menu screen
        GameState::Menu => {
            clear_background(WHITE);
            draw_text("HEllo idiots!", 100., 100., 20., BLUE);
        }

        //Credit screen
        GameState::Credit => {
            clear_background(YELLOW);
            draw_text("Game made by me: Loaf", SCREEN_WIDTH / 2., SCREEN_HEIGHT / 2., 30., GREEN);
        }

        //Ingame screen
        GameState::InGame => {
            clear_background(BLUE);

            let score = format!("Score: {}", game.score);

            draw_text(&score, 0., 10., 20., WHITE);

            draw_rectangle(game.player.pos.x, game.player.pos.y, game.player.size.x, game.player.size.y, BLACK);
            
            for bullets in &game.bullet {
                if bullets.active {
                    draw_rectangle(bullets.pos.x, bullets.pos.y, bullets.size.x, bullets.size.y, RED);
                }
            }

            if game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.health > 0 && game.score >= 10 {
                let boss_health = format!("Boss health: {}", game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.health);
                draw_text(&boss_health, SCREEN_WIDTH - 130., 10., 20., WHITE);

                draw_rectangle(game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.pos.x, game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.pos.y, game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.size.x, game.superultradupermegacoolshootingobjectforabsolutelynoreasonatall.size.y, RED);
            }

            for enemies in &game.enemy {
                if enemies.active {
                    draw_rectangle(enemies.pos.x, enemies.pos.y, enemies.size.x, enemies.size.y, WHITE);
                }
            }
        }

        GameState::EndGame => {
            //Win screen
            clear_background(RED);
            draw_text("ok", 100., 100., 20., BLACK);
            draw_text("You won or something like that I don't know lol", 100., 150., 20., BLUE);
            draw_text("Or", 150., 250., 20., YELLOW);
        }
    }
}

//check for collision
fn collision(x: &A, y: &A, z: &A, t: &A, a: &A, b: &A, c: &A, d: &A) -> bool {
    if *x < *a + *c && *y < *b + *d && *a < *x + *z && *b < *y + *t {true}
    else {false}
}