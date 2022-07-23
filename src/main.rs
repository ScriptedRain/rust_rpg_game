use std::io;
use rand::Rng;

static mut power: i32 = 0;
static mut ability: i32 = 1;
static mut gold: i32 = 0;

struct Enemy {
    epower: i32,
    health: i32,
    reward: i32,
}

fn main() {
    input("you're starting out on an adventure. E to train, W to move forward");
    loop {
       unsafe { action() };
    }
}

fn input(prompt: &str) -> String {
    println!("{prompt} :");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("parsing into string error");
    input = input.trim().parse().unwrap();
    return input;
}

unsafe fn action() {
    let mut rng = rand::thread_rng();
    // what is your action
    let action = input("What do you want to do?");
    if action == "e" {
        train();
    }
    else if action == "w" {
        walk();
    }
    else if action == "s" {
        SpitStats();
    }
    else {
        println!("Oh no you entered something incorrect try again");
    }
}

unsafe fn train() {
    println!("training...");
    let step = getRng(0, 601);
    println!("{power} + {}", step);
    power = power + step;
    println!("{}", power);
}


unsafe fn walk() {
    let res = getRng(0, 3);

    if res == 0 {
        let foundgold = getGold(1, 100);
        println!("You found gold! {}", foundgold);
        gold = gold + foundgold;
    }
    else if res == 1 {
        let enemy = spawnEnemy();
        fightEnemy(enemy.epower, enemy.health, enemy.reward);

    }
    else if res == 2 {
        println!("You saw nothing, you keep walking")
    }
}

fn spawnEnemy() -> Enemy {
    let mut enemy = Enemy {
        epower: getRng(200, 1000),
        health: getRng(400, 2000),
        reward: getGold(1000, 10000),
    };

   enemy.reward = (enemy.epower + enemy.health + enemy.reward) / 3;

    return enemy;
}

unsafe fn fightEnemy(epower: i32, health: i32, reward: i32) {
    println!("you encountered an enemy! power: {}, health: {}, reward: {}", epower, health, reward);
    println!("Your power level is : {power}");

    loop {
        let action = input("A to accept and attack, D to decline and run away...");

        if action == "a" {
            fightScene(epower, health, reward);
            break
        }

        else if action == "d" {
            println!("you are a coward, but maybe a smart one? you ran away");
            let luck = getRng(0, 2);
            if luck == 0 {
                fightScene(epower, health, reward);
            }

            break
        }
        else {
            println!("try again");
            break
        }
    }

}

unsafe fn fightScene(epower: i32, health: i32, reward: i32) {
    println!("the fight begins...");

    let luck = getRng(0, 3);
    if luck == 0 {
        if power > (epower + health) / 2 {
            println!("You Won! your reward is + {}", reward);
            power = power + epower * ability;
            gold = gold + reward * ability;
        }

        else if power < (epower + health) / 2 {
            println!("Oh no! You Died! All your stats are gone...");
            statReset();
        }
    }
    else if luck == 1 {
        println!("you attacked but they dodged your move!");

        let choice = input("choose your move carefully!");

        let luck2 = getRng(0, 2);
        if luck2 == 0 {
            println!("Oh no! You Died! All your stats are gone...");
            statReset();
        }
    }
    else if luck == 2 {
        println!("You killed the beast with one hit!!, well done. Your reward is + {}", reward);
        gold = gold + reward * ability;
        power = power + epower * ability;
        ability = ability + getRng(1, 11)
    }

}

unsafe fn SpitStats() {
    println!("________________________________________________________________________________");
    println!("power: {}", power);
    println!("ability: {}", ability);
    println!("gold: {}", gold);

}


// dev functions

fn getRng(min: i32, max: i32) -> i32{
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}

fn getGold(min: i32, max: i32) -> i32 {
    return getRng(1, 1001);
}

unsafe fn statReset() {
    power = 0;
    ability = 1;
    gold = 0;
}


// debugging functions
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
