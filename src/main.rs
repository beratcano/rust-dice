use clap::Parser;
use colored::*;
use rand::{random_bool, Rng};
use std::process;

#[derive(Parser, Debug)]
#[command(name = "Dice", version = "1.0", author = "Beratcan Özkan")]
struct Args {
    #[arg(required = true)]
    roll: Vec<String>,
}

fn main() {
  let args = Args::parse();
  let mut results = Vec::new();

  for arg in args.roll 
  {
    // Yazı Tura
    if arg.to_lowercase() == "yazıtura" || arg.to_lowercase() == "yt" || arg.to_lowercase() == "yazıtaur" {
      let result = if random_bool(0.5) {
        "Yazı 🪙".green().to_string()
      } else {
        "Tura 🪙".yellow().to_string()
      };
      println!("{}", result);
      continue;
    }
    // XdY+Z 
    if let Some((count, sides, modifier)) = parse_dice_expr(&arg) {
      let mut rng = rand::rng();
      let mut rolls = Vec::new();
      for _ in 0..count {
        rolls.push(rng.random_range(1..=sides));
      }
      let sum: i32 = rolls.iter().map(|&x| x as i32).sum();
      let total = sum + modifier;
      println!(
        "{} → Rolls: {:?} {}= {}",
        arg.cyan(),
        rolls,
        if modifier != 0 {
          format!("{} {} ", if modifier > 0 { "+" } else { "-" }, modifier.abs())
        } else {
          "".to_string()
        },
        total.to_string().bold()
      );
      continue;
    }
    // dX
    if let Some(sides) = parse_single_dice(&arg) {
      let roll = rand::random_range(1..=sides);
      results.push(roll);
      continue;
    }
    // Düz Sayı
    if let Ok(n) = arg.parse::<u32>() {
    let roll = rand::random_range(1..=n);
    println!("🎯 Random number (1-{}): {}", n, roll.to_string().bold());
    continue;
    }
    eprintln!("⚠️ Unknown input: {}", arg.red());
    process::exit(1);
  }
  if !results.is_empty() {
    println!("🎲 You rolled: {:?}", results);
  }
}

fn parse_dice_expr(input: &str) -> Option<(u32,u32,i32)> {
  let input = input.to_lowercase();
  let parts = input.split("d").collect::<Vec<&str>>();
  if parts.len() != 2 {
    return None;
  }

  let count = parts[0].parse::<u32>().unwrap_or(1);

  let (sides, modifier) = if parts[1].contains("+") {
    let parts2 = parts[1].split("+").collect::<Vec<_>>();
    (parts2[0].parse().ok()?, parts2[1].parse().ok()?)
  } else if parts[1].contains("-") {
    let parts2 = parts[1].split("-").collect::<Vec<_>>();
    (parts2[0].parse().ok()?, -(parts2[1].parse::<i32>().ok()?))
  } else {
    (parts[1].parse().ok()?, 0)
  };
  Some((count, sides, modifier))
}

fn parse_single_dice(input: &str) -> Option<u32> {
  if let Some(stripped) = input.strip_prefix("d"){
    return stripped.parse::<u32>().ok();
  }
  None
}