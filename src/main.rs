
use sdl2::event::Event;
use sdl2::event::Event::Quit;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f32::consts::PI;

const W: u32 = 800;
const H: u32 = 600;

// fn fill_circle_line() {
//     for angle in 0..360 {
//         let rad = angle as f32 * PI / 180.0;

//         let x = self.x - (RADIUS * rad.cos()) as i32;
//         let y = self.y - (RADIUS * rad.sin()) as i32;

//         canvas.draw_line((self.x, self.y), (x, y)).unwrap();
//     }
// }

struct Vec2 {
    x: i32,
    y: i32,
}

struct Circle {
    center: Point,
    radius: u32,
    color: Color,
}

impl Circle {
    fn render(&self, canvas: &mut Canvas<Window>) {
        let base_triangle = Triangle {
            color: self.color,
            p1: self.center,
            p2: Point::new(self.center.x, self.center.y + self.radius as i32),
            p3: Point::new(self.center.x - 1, self.center.y + self.radius as i32),
        };
        for angle in 1..360 {
            let mut tri = base_triangle.clone();
            tri.rotate(self.center, angle as f32);
            tri.render(canvas);
        }
    }
}

#[derive(Clone, Copy)]
struct Triangle {
    color: Color,
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    fn rotate(&mut self, center: Point, angle: f32) {
        let rad: f32 = angle * PI / 180.0;
        let p1 = self.p1 - center;
        let p2 = self.p2 - center;
        let p3 = self.p3 - center;

        self.p1.x = center.x + (rad.cos() * p1.x as f32 - rad.sin() * p1.y as f32).round() as i32;
        self.p2.x = center.x + (rad.cos() * p2.x as f32 - rad.sin() * p2.y as f32).round() as i32;
        self.p3.x = center.x + (rad.cos() * p3.x as f32 - rad.sin() * p3.y as f32).round() as i32;

        self.p1.y = center.y + (rad.sin() * p1.x as f32 + rad.cos() * p1.y as f32).round() as i32;
        self.p2.y = center.y + (rad.sin() * p2.x as f32 + rad.cos() * p2.y as f32).round() as i32;
        self.p3.y = center.y + (rad.sin() * p3.x as f32 + rad.cos() * p3.y as f32).round() as i32;
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        let mut edges = Vec::<Point>::new();
        for (mut p1, mut p2) in [(self.p1, self.p2), (self.p2, self.p3), (self.p3, self.p1)] {
            if p1.y == p2.y {
                continue;
            }
            if p1.y > p2.y {
                std::mem::swap(&mut p1, &mut p2);
            }
            let m = ((p2.x - p1.x) as f32 / (p2.y - p1.y) as f32) as f32;
            let c = p1.x as f32;
            for (i, y) in (p1.y..p2.y).enumerate() {
                let x = (m * i as f32 + c).round();
                let point = Point::new(x as i32, y);
                edges.push(point);
            }
        }
        edges.sort_unstable_by(|p1, p2| p1.y.cmp(&p2.y));
        canvas.set_draw_color(self.color);
        canvas.draw_lines(edges.as_slice()).unwrap();
    }
}

trait Entity {
    fn update(&mut self, dt: f32);
    fn render(&self, canvas: &mut Canvas<Window>);
}

struct Player {
    object: Circle,
    velocity: Vec2,
}

impl Entity for Player {
    fn update(&mut self, dt: f32) {
        if self.object.center.y > (H - self.object.radius) as i32 {
            self.object.center.y = (H - self.object.radius) as i32;
            self.velocity.y *= -1;
        } else if self.object.center.y < self.object.radius as i32 {
            self.object.center.y = self.object.radius as i32;
            self.velocity.y *= -1;
        }

        if self.object.center.x > (W - self.object.radius) as i32 {
            self.object.center.x = (W - self.object.radius) as i32;
            self.velocity.x *= -1;
        } else if self.object.center.x < self.object.radius as i32 {
            self.object.center.x = self.object.radius as i32;
            self.velocity.x *= -1;
        }

        self.object.center.y += (dt * self.velocity.y as f32) as i32;
        self.object.center.x += (dt * self.velocity.x as f32) as i32;
    }

    fn render(&self, canvas: &mut Canvas<Window>) {
        self.object.render(canvas);
    }
}

fn main() {
    let ctx = sdl2::init().unwrap();
    let timer_subsystem = ctx.timer().unwrap();

    let video = ctx.video().unwrap();

    let window = video
        .window("kafatopu", W, H)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut event_pump = ctx.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let circle = Circle {
        center: Point::new(100, 100),
        radius: 30,
        color: Color::GREEN,
    };
    let mut player = Player {
        object: circle,
        velocity: Vec2 { x: 0, y: 0 },
    };

    let mut last: f32 = 0.0;
    const FPS: f32 = 60.0;
    const SPF: f32 = 1.0 / FPS;
    let mut is_paused = false;

    // let mut triangle = Triangle {
    //     color: Color::RED,
    //     p1: Point::new(100, 100),
    //     p2: Point::new(100, 130),
    //     p3: Point::new(130, 100),
    // };

    loop {
        // event handling
        for event in event_pump.poll_iter() {
            match event {
                Quit { .. } => return,
                Event::KeyDown {
                    scancode: Some(Scancode::P),
                    ..
                } => is_paused = !is_paused,

                Event::KeyDown {
                    scancode: Some(sc), ..
                } => match sc {
                    Scancode::Down => player.velocity.y = 100,
                    Scancode::Up => player.velocity.y = -100,
                    Scancode::Right => player.velocity.x = 100,
                    Scancode::Left => player.velocity.x = -100,
                    _ => {}
                },
                Event::KeyUp {
                    scancode: Some(sc), ..
                } => match sc {
                    Scancode::Down => player.velocity.y = 0,
                    Scancode::Up => player.velocity.y = 0,
                    Scancode::Right => player.velocity.x = 0,
                    Scancode::Left => player.velocity.x = 0,
                    _ => {}
                },
                _ => {}
            }
        }
        // timer
        let ticks = timer_subsystem.ticks() as f32;
        let dt = (ticks - last) / 1000.0;
        if dt < SPF {
            std::thread::sleep(std::time::Duration::from_secs_f32(SPF - dt));
        }
        last = ticks;
        if is_paused {
            continue;
        }

        // logic
        player.update(dt);

        // render
        canvas.set_draw_color(Color::BLUE);
        canvas.clear();

        player.render(&mut canvas);
        // triangle.render(&mut canvas);
        // triangle.rotate(Point::new(100, 100), 1.0);

        canvas.present();
    }
}
