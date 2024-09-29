use std::fmt::{Display, Formatter};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};

use eframe::egui;
use egui::Color32;
use egui::widgets::Label;
use egui::widget_text::RichText;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::thread_rng;
use rand::prelude::SliceRandom;

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades
}

impl Suit {
    fn to_string(&self)->String{
        match *self {
            Suit::Diamonds => "♦".to_string(),
            Suit::Clubs => "♣".to_string(),
            Suit::Hearts=> "♥".to_string(),
            Suit::Spades => "♠".to_string()
        }
    }
    fn to_name(&self)->String{
        match *self {
            Suit::Diamonds => "diamonds".to_string(),
            Suit::Clubs => "clubs".to_string(),
            Suit::Hearts=> "hearts".to_string(),
            Suit::Spades => "spades".to_string()
        }
    }
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Rank {
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Ace
}

impl Rank {
    fn to_string(&self)->String {
        match *self {
            Rank::King => "K".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Ten=> "10".to_string(),
            Rank::Nine=> "9".to_string(),
            Rank::Eight=> "8".to_string(),
            Rank::Seven=> "7".to_string(),
            Rank::Six=> "6".to_string(),
            Rank::Five=> "5".to_string(),
            Rank::Four=> "4".to_string(),
            Rank::Three=> "3".to_string(),
            Rank::Two=> "2".to_string(),
            Rank::Ace=> "A".to_string()
        }
    }
    fn to_name(&self)->String {
        match *self {
            Rank::King => "K".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Ten=> "10".to_string(),
            Rank::Nine=> "09".to_string(),
            Rank::Eight=> "08".to_string(),
            Rank::Seven=> "07".to_string(),
            Rank::Six=> "06".to_string(),
            Rank::Five=> "05".to_string(),
            Rank::Four=> "04".to_string(),
            Rank::Three=> "03".to_string(),
            Rank::Two=> "02".to_string(),
            Rank::Ace=> "A".to_string()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "[ {} {} ]",self.rank.to_string(),self.suit.to_string())
    }
}

impl Card {
    pub fn to_string(&self)->String {
        format!("[ {} {} ]",self.rank.to_string(),self.suit.to_string())
    }

    pub fn new(rank:Rank, suit:Suit) -> Self {
        Self {
            rank: rank,
            suit: suit
        }
    }
}


fn sum(cards: &Vec<Card>)->i32{
    let mut sum = 0;
    for c in cards{
        match c.rank {
            Rank::King => sum +=10,
            Rank::Queen => sum +=10,
            Rank::Jack => sum +=10,
            Rank::Ten=> sum +=10,
            Rank::Nine=> sum +=9,
            Rank::Eight=> sum +=8,
            Rank::Seven=> sum +=7,
            Rank::Six=> sum +=6,
            Rank::Five=> sum +=5,
            Rank::Four=> sum +=4,
            Rank::Three=> sum +=3,
            Rank::Two=> sum +=2,
            Rank::Ace=> sum +=11
        }
    }
    sum
}

pub trait PresentationLayer{
    fn show(&self, cards_player: &Vec<Card>, sum_player:i32,
            cards_bank: &Vec<Card>, sum_bank:i32)->();
    fn another_card(&self)->bool;
    fn show_winner(&self,player_win:bool)->();
}

struct PresentationLayerEgui{
    sender: Sender<GameState>,
    receiver: Receiver<Answer>
}

impl PresentationLayerEgui{
    fn new(sender:Sender<GameState>, receiver:Receiver<Answer>)->Self{
        Self{
            sender:sender,
            receiver:receiver
        }

    }
}

impl PresentationLayer for PresentationLayerEgui {
    fn show(&self, cards_player: &Vec<Card>, sum_player: i32, cards_bank: &Vec<Card>, sum_bank: i32) -> () {
        let mut c_cards_player: Vec<Card> = Vec::new();
        for c in cards_player{
            c_cards_player.push(c.clone());
        };
        let mut c_cards_bank: Vec<Card> = Vec::new();
        for c in cards_bank{
            c_cards_bank.push(c.clone());
        };
        self.sender.send(GameState{
            sum_bank: sum_bank,
            sum_player:sum_player,
            cards_bank: c_cards_bank,
            cards_player: c_cards_player,
            wait_for_answer:false,
            message:"".to_string()
        });
    }
    fn another_card(&self) -> bool {
        self.sender.send(GameState{
            sum_bank: -1,
            sum_player:-1,
            cards_bank: Vec::new(),
            cards_player: Vec::new(),
            wait_for_answer:true,
            message:"".to_string()
        });
        self.receiver.recv().unwrap().another_card
    }
    fn show_winner(&self, player_win: bool) -> () {
        let message = if player_win {
            "Player win".to_string()
        } else {
            "Bank win".to_string()
        };
        self.sender.send(GameState{
            sum_bank: -1,
            sum_player:-1,
            cards_bank: Vec::new(),
            cards_player: Vec::new(),
            wait_for_answer:true,
            message:message.clone()
        });
    }
}


struct GameState{
    sum_bank: i32,
    sum_player: i32,
    cards_player: Vec<Card>,
    cards_bank: Vec<Card>,
    wait_for_answer: bool,
    message:String
}

fn main() {
    let (sender_game, receiver_gui) = mpsc::channel();
    let (sender_gui, receiver_game) = mpsc::channel();

    let presentation_layer = PresentationLayerEgui::new(sender_game, receiver_game);
    thread::spawn(|| {
        run_game(presentation_layer);
    });
    run_gui(receiver_gui,sender_gui);
}

fn run_game(presentation_layer:PresentationLayerEgui) {
    let mut cards = create_cards();
    let mut cards_player:Vec<Card> = Vec::new();
    let mut cards_bank:Vec<Card> = Vec::new();

    cards_player.push(cards.pop().unwrap());
    cards_bank.push(cards.pop().unwrap());

    let mut player_win = true;

    presentation_layer.show(&cards_player, sum(&cards_player),
                            &cards_bank, sum(&cards_bank));

    cards_player.push(cards.pop().unwrap());

    loop {
        presentation_layer.show(&cards_player, sum(&cards_player),
                                &cards_bank, sum(&cards_bank));

        if sum(&cards_player) > 21 {
            player_win = false;
            break;
        }

        if presentation_layer.another_card() {
            cards_player.push(cards.pop().unwrap());
        } else {
            break;
        }
    }
    cards_bank.push(cards.pop().unwrap());

    loop {
        if player_win == false || sum(&cards_bank) > 21 {
            break
        }
        presentation_layer.show(&cards_player, sum(&cards_player),
                                &cards_bank, sum(&cards_bank));
        if sum(&cards_bank) < 17 {
            cards_bank.push(cards.pop().unwrap());
        } else {
            break;
        }
    }
    if sum(&cards_bank) > 21 {
        player_win = true;
    } else if sum(&cards_bank) > sum(&cards_player) {
        player_win = false;
    }
    presentation_layer.show_winner(player_win);
}

fn create_cards() -> Vec<Card>{
    let mut cards = Vec::<Card>::new();

    for r in Rank::iter() {
        for s in Suit::iter() {
            cards.push(Card::new(r,s))
        }
    };

    let mut rng = thread_rng();
    cards.shuffle(&mut rng);

    cards

}

fn run_gui(receiver:Receiver<GameState>,sender:Sender<Answer>) -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "twentyone",
        options,
        Box::new(|_cc| Box::new(GameApp{receiver,sender,game_state:GameState{sum_bank:0,sum_player:0,
            cards_bank:Vec::new(),cards_player:Vec::new(),wait_for_answer:false, message: "".to_string()}})),
    )
}

struct Answer {
    another_card:bool
}

struct GameApp {
    receiver:Receiver<GameState>,
    sender:Sender<Answer>,
    game_state:GameState
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        match self.receiver.try_recv() {
            Ok(value) => {
                if value.message.len() > 0 {
                    self.game_state.message = value.message.clone();
                } else {
                    self.game_state.message = "".to_string();
                }
                if value.wait_for_answer {
                    self.game_state.wait_for_answer = true
                } else {
                    self.game_state = value;
                }

            }
            _ => {}
        }

        let my_frame = egui::containers::Frame {
            inner_margin: egui::style::Margin { left: 4., right: 4., top: 4., bottom: 4. },
            outer_margin: egui::style::Margin { left: 4., right: 4., top: 4., bottom: 4. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 1.0, color: Color32::YELLOW },
            fill: Color32::LIGHT_GREEN,
            stroke: egui::Stroke::new(2.0, Color32::GOLD),
        };

        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
            ui.add(Label::new(RichText::new(format!("Bank {}", self.game_state.sum_bank)).heading().color(Color32::BLACK)));
            ui.horizontal(|ui| {
                for c in &self.game_state.cards_bank {
                    let mut texture_store: Option<egui::TextureHandle>=Option::None;
                    let file_name = format!("cards/card_{}_{}.png",c.suit.to_name(),c.rank.to_name());
                    let texture: &egui::TextureHandle = texture_store.get_or_insert_with(|| {
                        // Load the texture only once.
                        ui.ctx().load_texture(
                            "my-image",
                            load_image_from_path(file_name).unwrap(),
                            Default::default()
                        )
                    });

                    // Show the image:
                    ui.image(texture, texture.size_vec2());
                }
            });
            ui.add(Label::new(RichText::new(format!("Player {}", self.game_state.sum_player)).heading().color(Color32::BLACK)));
            ui.horizontal(|ui| {
                for c in &self.game_state.cards_player {
                    let mut texture_store: Option<egui::TextureHandle>=Option::None;
                    let file_name = format!("cards/card_{}_{}.png",c.suit.to_name(),c.rank.to_name());
                    let texture: &egui::TextureHandle = texture_store.get_or_insert_with(|| {
                        // Load the texture only once.
                        ui.ctx().load_texture(
                            "my-image",
                            load_image_from_path(file_name).unwrap(),
                            Default::default()
                        )
                    });

                    // Show the image:
                    ui.image(texture, texture.size_vec2());
                }
            });
            ui.add_space(20.);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("Hit").clicked() {
                    if self.game_state.wait_for_answer{
                        self.sender.send(Answer{
                            another_card:true
                        });
                    }
                }
                if ui.button("Stay").clicked() {
                    if self.game_state.wait_for_answer{
                        self.sender.send(Answer{
                            another_card:false
                        });
                    }
                }
                ui.add(Label::new(RichText::new(format!(" {}", self.game_state.message)).heading().color(Color32::BLACK)));
            });
        });
    }
}

fn load_image_from_path(path: String) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}