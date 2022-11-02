use core::num::dec2flt::number::Number;
use std::vec::IntoIter;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Serialize, Deserialize};
use strum::{EnumVariantNames, VariantNames};

#[derive(EnumVariantNames, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum CardNumber {
	Two = 2,
	Three = 3,
	Four = 4,
	Five = 5,
	Six = 6,
	Seven = 7,
	Eight = 8,
	Nine = 9,
	Ten = 10,
	Jack = 11,
	Queen = 12,
	King = 13,
	Ace = 14,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Card {
	Hearts(CardNumber),
	Diamonds(CardNumber),
	Spades(CardNumber),
	Clubs(CardNumber),
	RedJoker,
	BlackJoker
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Hand (Vec<Card>);

impl Hand {
	pub fn all_cards () -> Self {
		let mut list = Vec::with_capacity(54);
		for n in Number::VARIANTS {
			list.push(Card::Hearts(n));
			list.push(Card::Diamonds(n));
			list.push(Card::Spades(n));
			list.push(Card::Clubs(n));
		}
		list.push(Card::RedJoker);
		list.push(Card::BlackJoker);

		Self(list)
	}

	pub fn from_cards (cards: Vec<Card>) -> Self {
		Self(cards)
	}

	pub fn get_cards (&self) -> Vec<Card> {
		self.0.clone()
	}

	pub fn draw_from_top (&mut self) -> Option<Card> {
		if self.0.is_empty() {
			None
		} else {
			Some(self.0.remove(0))
		}
	}

	pub fn shuffle (&mut self) {
		self.0.shuffle(&mut thread_rng());
	}
}

impl IntoIterator for Hand {
	type Item = Card;
	type IntoIter = IntoIter<Card>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}