#[derive(Debug)]
/// Событие лифта на которое должен реагировать контроллер.
enum Event {
    CarArrived(Floor),
    CarDoorStateUpdated(CarDoorState),
    ButtonPressed(Button),
}

type Floor = i32;

#[derive(Debug)]
/// Состояние дверей лифта
enum CarDoorState {
    Opened,
    Closed,
}

#[derive(Debug)]
enum Button {
    LobbyCall(Floor, Direction),
    CarFloor(Floor)
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// Кабина приехала на заданный этаж.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// Двери кабины открыты.
fn car_door_opened() -> Event {
    Event::CarDoorStateUpdated(CarDoorState::Opened)
}

/// Двери кабины закрыты.
fn car_door_closed() -> Event {
    Event::CarDoorStateUpdated(CarDoorState::Closed)
}

/// Кнопка вызова лифта нажата на заданном этаже.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(floor, dir))
}

/// Кнопка этажа нажата в кабине лифта.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
}

fn main() {
    println!(
        "Пассажир на первом этаже нажал кнопку вызова: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("Лифт приехал на первый этаж: {:?}", car_arrived(0));
    println!("Дверь лифта открылась: {:?}", car_door_opened());
    println!(
        "Пассажир нажал кнопку третьего этажа: {:?}",
        car_floor_button_pressed(3)
    );
    println!("Двери лифта закрылись: {:?}", car_door_closed());
    println!("Лифт прибыл на третий этаж: {:?}", car_arrived(3));
}