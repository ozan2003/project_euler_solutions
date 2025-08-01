#![feature(iter_array_chunks)]
use std::env::current_dir;
use std::fs::File;
use std::io::{BufRead, BufReader};

use poker::{Card, Hand};
use project_euler::project_euler_solution;

project_euler_solution!(054);

/// # Poker Hands
///
/// In the card game poker, a hand consists of five cards and are ranked, from
/// lowest to highest, in the following way:
///
/// * High Card: Highest value card.
///
/// * One Pair: Two cards of the same value.
///
/// * Two Pairs: Two different pairs.
///
/// * Three of a Kind: Three cards of the same value.
///
/// * Straight: All cards are consecutive values.
///
/// * Flush: All cards of the same suit.
///
/// * Full House: Three of a kind and a pair.
///
/// * Four of a Kind: Four cards of the same value.
///
/// * Straight Flush: All cards are consecutive values of same suit.
///
/// * Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
///
/// The cards are valued in the order:
///  2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
///
/// If two players have the same ranked hands then the rank made up of the
/// highest value wins; for example, a pair of eights beats a pair of fives (see
/// example 1 below). But if two ranks tie, for example, both players have a
/// pair of queens, then highest cards in each hand are compared (see example 4
/// below); if the highest cards tie then the next highest cards are compared,
/// and so on.
///
/// Consider the following five hands dealt to two players:
///
/// | Hand  | Player 1       | Player 2       | Winner   |
/// |-------|----------------|----------------|----------|
/// | 1     | 5H 5C 6S 7S KD | 2C 3S 8S 8D TD | Player 2 |
/// | 2     | 5D 8C 9S JS AC | 2C 5C 7D 8S QH | Player 1 |
/// | 3     | 2D 9C AS AH AC | 3D 6D 7D TD QD | Player 1 |
/// | 4     | 4D 6S 9H QH QC | 3D 6D 7H QD QS | Player 2 |
/// | 5     | 2H 2D 4C 4D 4S | 3C 3D 3S 9S 9D | Player 1 |
///
/// The file, poker.txt, contains one-thousand random hands dealt to two
/// players. Each line of the file contains ten cards (separated by a single
/// space): the first five are Player 1's cards and the last five are Player 2's
/// cards. You can assume that all hands are valid (no invalid characters or
/// repeated cards), each player's hand is in no specific order, and in each
/// hand there is a clear winner.
///
/// How many hands does Player 1 win?
fn project_euler_054() -> usize
{
    let proj_dir = current_dir().unwrap();

    let file =
        File::open(proj_dir.join("data/054.txt")).expect("Failed to open file");
    let buf = BufReader::new(file);

    let player_cards: Vec<(Hand, Hand)> = buf
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(|line| parse_cards(&line))
        .collect();

    player_cards
        .into_iter()
        .filter(|(p1, p2)| p1 > p2)
        .count()
}

/// Parse the cards 5 by 5, each 5 cards are a hand for a player.
///
/// # Arguments
/// * `card_list` - A string containing 10 cards separated by spaces.
///
/// # Returns
/// A tuple containing two `Hand`s, the first for Player 1 and the second for
/// Player 2.
fn parse_cards(card_list: &str) -> (Hand, Hand)
{
    let mut iter = card_list
        .split_whitespace()
        .map(Card::new)
        .array_chunks::<5>();

    // Unwrap is safe here because we know there are exactly 10 cards in the
    // input
    let hand_p1 = Hand::new(iter.next().unwrap());
    let hand_p2 = Hand::new(iter.next().unwrap());

    (hand_p1, hand_p2)
}

mod poker
{
    use std::cmp::Ordering;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    /// The suit of a card
    pub enum Suit
    {
        Hearts,
        Diamonds,
        Clubs,
        Spades,
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
    /// The value of a card
    pub enum Value
    {
        Two = 2,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    /// A card comprised of a value and a suit
    pub struct Card
    {
        pub value: Value,
        pub suit: Suit,
    }

    impl Card
    {
        /// Creates a new `Card` from a string representation.
        ///
        /// # Arguments
        /// * `text` - A string slice representing the card, e.g., "2H" for 2 of
        ///   Hearts.
        ///
        /// # Returns
        /// A new `Card` instance with the corresponding value and suit.
        pub fn new(text: &str) -> Self
        {
            let (value, suit) = text.split_at(text.len() - 1);

            let suit = match suit
            {
                "H" => Suit::Hearts,
                "D" => Suit::Diamonds,
                "C" => Suit::Clubs,
                "S" => Suit::Spades,
                _ => panic!("Invalid suit, {suit}"),
            };

            let value = match value
            {
                "2" => Value::Two,
                "3" => Value::Three,
                "4" => Value::Four,
                "5" => Value::Five,
                "6" => Value::Six,
                "7" => Value::Seven,
                "8" => Value::Eight,
                "9" => Value::Nine,
                "T" => Value::Ten,
                "J" => Value::Jack,
                "Q" => Value::Queen,
                "K" => Value::King,
                "A" => Value::Ace,
                _ => panic!("Invalid value, {value}"),
            };

            Self { value, suit }
        }
    }

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
    /// Enum representing the rank of a hand
    pub enum HandRank
    {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        Straight,
        Flush,
        FullHouse,
        FourOfAKind,
        StraightFlush,
        RoyalFlush,
    }

    /// A poker hand consisting of 5 cards.
    pub struct Hand([Card; 5]);

    impl Hand
    {
        /// Creates a new `Hand` with exactly 5 cards.
        pub const fn new(cards: [Card; 5]) -> Self
        {
            Self(cards)
        }

        /// Returns an iterator over the values of the cards in the hand.
        pub fn values(&self) -> impl Iterator<Item = Value>
        {
            self.0.iter().map(|card| card.value)
        }

        /// One pair means two cards of the same value.
        ///
        /// # Returns
        /// `true` if the hand contains exactly one pair, `false` otherwise.
        pub fn is_one_pair(&self) -> bool
        {
            let counts = self.value_counts();

            counts
                .values()
                .filter(|&&count| count == 2)
                .count() ==
                1
        }

        /// Two pair means two different pairs of cards.
        ///
        /// # Returns
        /// `true` if the hand contains exactly two pairs, `false` otherwise.
        pub fn is_two_pair(&self) -> bool
        {
            let counts = self.value_counts();
            let mut pair_count = 0;
            // Shouldn't be three of a kind or four of a kind.
            let mut is_triplet_or_higher = false;

            for &count in counts.values()
            {
                match count
                {
                    2 => pair_count += 1,
                    3 | 4 => is_triplet_or_higher = true,
                    _ =>
                    {},
                }
            }

            !is_triplet_or_higher && pair_count == 2
        }

        /// Three of a kind means three cards of the same value.
        ///
        /// # Returns
        /// `true` if the hand contains exactly one three of a kind, `false`
        /// otherwise.
        pub fn is_three_of_a_kind(&self) -> bool
        {
            let counts = self.value_counts();
            let mut has_triplet = false;
            let mut has_quad = false; // Shouldn't be four of a kind.

            for &count in counts.values()
            {
                match count
                {
                    3 => has_triplet = true,
                    4 => has_quad = true,
                    _ =>
                    {},
                }
            }

            has_triplet && !has_quad
        }

        /// Straight means five consecutive values with any suit.
        ///
        /// # Returns
        /// `true` if the hand contains a straight, `false` otherwise.
        pub fn is_straight(&self) -> bool
        {
            let unique_values = self.unique_ascending_values();

            // Must have exactly 5 unique values for a straight
            if unique_values.len() != 5
            {
                return false;
            }

            // Check for regular straight (consecutive values)
            let is_consecutive = unique_values
                .windows(2)
                .all(|pair| pair[1] as u8 == pair[0] as u8 + 1);

            if is_consecutive
            {
                return true;
            }

            // Check for Ace-low straight (A-2-3-4-5)
            if unique_values ==
                [
                    Value::Two,
                    Value::Three,
                    Value::Four,
                    Value::Five,
                    Value::Ace,
                ]
            {
                return true;
            }

            false
        }

        /// Flush means all cards of the same suit.
        ///
        /// # Returns
        /// `true` if the hand contains a flush, `false` otherwise.
        pub fn is_flush(&self) -> bool
        {
            let first_suit = &self.0[0].suit;

            self.0
                .iter()
                .skip(1)
                .all(|card| card.suit == *first_suit)
        }

        /// Full house means three of a kind and one pair.
        ///
        /// # Returns
        /// `true` if the hand contains a full house, `false` otherwise.
        pub fn is_full_house(&self) -> bool
        {
            let counts = self.value_counts();
            let mut has_triplet = false;
            let mut has_pair = false;

            for &count in counts.values()
            {
                match count
                {
                    2 => has_pair = true,
                    3 => has_triplet = true,
                    _ =>
                    {},
                }
            }

            has_triplet && has_pair && counts.len() == 2 // Exactly 2 different values
        }

        /// Four of a kind means four cards of the same value.
        ///
        /// # Returns
        /// `true` if the hand contains exactly one four of a kind, `false`
        /// otherwise.
        pub fn is_four_of_a_kind(&self) -> bool
        {
            let counts = self.value_counts();

            counts.values().any(|&count| count == 4)
        }

        /// Straight flush means five consecutive values of the same suit.
        ///
        /// # Returns
        /// `true` if the hand contains a straight flush, `false` otherwise.
        pub fn is_straight_flush(&self) -> bool
        {
            self.is_straight() && self.is_flush()
        }

        /// Royal flush means a straight flush with the highest values.
        ///
        /// # Returns
        /// `true` if the hand contains a royal flush, `false` otherwise.
        pub fn is_royal_flush(&self) -> bool
        {
            if !self.is_flush()
            {
                return false;
            }

            let mut values: Vec<Value> = self
                .0
                .iter()
                .map(|card| card.value)
                .collect();
            values.sort_unstable();

            values ==
                [
                    Value::Ten,
                    Value::Jack,
                    Value::Queen,
                    Value::King,
                    Value::Ace,
                ]
        }

        /// Helper method to count the occurrences of each value in the hand
        ///
        /// # Returns
        /// A `HashMap` where keys are `Value`s and values are the count of
        /// each value in the hand.
        fn value_counts(&self) -> HashMap<Value, usize>
        {
            let mut counts = HashMap::new();

            for card in &self.0
            {
                counts
                    .entry(card.value)
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }

            counts
        }

        /// Get unique sorted values from the hand
        ///
        /// # Returns
        /// A `Vec<Value>` containing unique values sorted in ascending order.
        fn unique_ascending_values(&self) -> Vec<Value>
        {
            let mut values: Vec<Value> = self
                .0
                .iter()
                .map(|card| card.value)
                .collect();

            values.sort_unstable();
            values.dedup();

            values
        }

        /// Determines the rank of this hand
        pub fn get_rank(&self) -> HandRank
        {
            if self.is_royal_flush()
            {
                HandRank::RoyalFlush
            }
            else if self.is_straight_flush()
            {
                HandRank::StraightFlush
            }
            else if self.is_four_of_a_kind()
            {
                HandRank::FourOfAKind
            }
            else if self.is_full_house()
            {
                HandRank::FullHouse
            }
            else if self.is_flush()
            {
                HandRank::Flush
            }
            else if self.is_straight()
            {
                HandRank::Straight
            }
            else if self.is_three_of_a_kind()
            {
                HandRank::ThreeOfAKind
            }
            else if self.is_two_pair()
            {
                HandRank::TwoPair
            }
            else if self.is_one_pair()
            {
                HandRank::OnePair
            }
            else
            {
                HandRank::HighCard
            }
        }

        /// Compare this hand with another hand to determine the winner
        ///
        /// # Returns
        /// `Ordering::Greater` if self wins, `Ordering::Less` if other wins,
        /// `Ordering::Equal` for tie
        pub fn compare(&self, other: &Hand) -> Ordering
        {
            let self_rank = self.get_rank();
            let other_rank = other.get_rank();

            // First compare by hand rank
            match self_rank.cmp(&other_rank)
            {
                Ordering::Equal =>
                {
                    // If same rank, compare by specific tie-breaking rules
                    self.compare_same_rank(other, &self_rank)
                },
                other_ordering => other_ordering,
            }
        }

        /// Handle tie-breaking when the both hands have the same rank
        ///
        /// # Arguments
        /// * `other` - The other hand to compare to
        /// * `rank` - The rank of the hand
        ///
        /// # Returns
        /// The ordering of the comparison between the two hands
        /// `Ordering::Equal` if the hands are tied, `Ordering::Greater` if
        /// the first hand wins, `Ordering::Less` if the second hand wins
        fn compare_same_rank(&self, other: &Hand, rank: &HandRank) -> Ordering
        {
            match *rank
            {
                // Always tie
                HandRank::RoyalFlush => Ordering::Equal,

                // Compare the highest card
                HandRank::StraightFlush | HandRank::Straight =>
                {
                    self.compare_straight_high_card(other)
                },

                // Compare the quad value, then the kicker
                HandRank::FourOfAKind => self.compare_four_of_a_kind(other),

                // Compare the triplet value, then the pair value
                HandRank::FullHouse => self.compare_full_house(other),

                // Compare all cards in descending order (like High Card)
                HandRank::Flush | HandRank::HighCard =>
                {
                    self.compare_high_cards(other)
                },

                // Compare the triplet value, then the remaining 2 kickers
                HandRank::ThreeOfAKind => self.compare_three_of_a_kind(other),

                // Compare the higher pair, then the lower pair, then the kicker
                HandRank::TwoPair => self.compare_two_pair(other),

                // Compare the pair value first, then the remaining 3 kicker
                // cards.
                HandRank::OnePair => self.compare_one_pair(other),
            }
        }

        /// Compare the highest card in a straight
        ///
        /// # Arguments
        /// * `other` - The other hand to compare to
        ///
        /// # Returns
        /// The ordering of the comparison between the two hands
        /// `Ordering::Equal` if the hands are tied, `Ordering::Greater` if
        /// the first hand wins, `Ordering::Less` if the second hand wins
        fn compare_straight_high_card(&self, other: &Hand) -> Ordering
        {
            // For straights, compare the highest card (but handle Ace-low
            // special case)
            let self_high = self.get_straight_high_card();
            let other_high = other.get_straight_high_card();
            self_high.cmp(&other_high)
        }

        /// Get the highest card in a straight
        ///
        /// # Returns
        /// The highest card in a straight
        fn get_straight_high_card(&self) -> Value
        {
            let values = self.unique_ascending_values();
            // Check for Ace-low straight (A-2-3-4-5)
            if values ==
                [
                    Value::Two,
                    Value::Three,
                    Value::Four,
                    Value::Five,
                    Value::Ace,
                ]
            {
                Value::Five // In Ace-low straight, 5 is the high card
            }
            else
            {
                *values.last().unwrap() // Highest value
            }
        }

        /// Compare four of a kind with another
        ///
        /// # Arguments
        /// * `other` - The other hand to compare to
        ///
        /// # Returns
        /// The ordering of the comparison between the two hands
        /// `Ordering::Equal` if the hands are tied, `Ordering::Greater` if
        /// the first hand wins, `Ordering::Less` if the second hand wins
        fn compare_four_of_a_kind(&self, other: &Hand) -> Ordering
        {
            let self_quad = self.get_n_of_a_kind_value(4);
            let other_quad = other.get_n_of_a_kind_value(4);
            self_quad.cmp(&other_quad)
        }

        /// Compare full house with another
        ///
        /// # Arguments
        /// * `other` - The other hand to compare to
        ///
        /// # Returns
        /// The ordering of the comparison between the two hands
        /// `Ordering::Equal` if the hands are tied, `Ordering::Greater` if
        /// the first hand wins, `Ordering::Less` if the second hand wins
        fn compare_full_house(&self, other: &Hand) -> Ordering
        {
            // Compare three of a kind first
            let self_triple = self.get_n_of_a_kind_value(3);
            let other_triple = other.get_n_of_a_kind_value(3);

            match self_triple.cmp(&other_triple)
            {
                Ordering::Equal =>
                {
                    // Compare the remaining 2 kickers
                    // They're also two of a kind, so we can use the same logic
                    let self_kicker = self.get_n_of_a_kind_value(2);
                    let other_kicker = other.get_n_of_a_kind_value(2);
                    self_kicker.cmp(&other_kicker)
                },
                other_ordering => other_ordering,
            }
        }

        /// Compare three of a kind with another
        ///
        /// # Arguments
        /// * `other` - The other hand to compare to
        ///
        /// # Returns
        /// The ordering of the comparison between the two hands
        fn compare_three_of_a_kind(&self, other: &Hand) -> Ordering
        {
            // Get the value of the triplet first
            let self_triple = self.get_n_of_a_kind_value(3);
            let other_triple = other.get_n_of_a_kind_value(3);

            match self_triple.cmp(&other_triple)
            {
                Ordering::Equal => self.compare_high_cards(other),
                other_ordering => other_ordering,
            }
        }

        /// Compare two pair with another
        ///
        /// # Arguments
        /// * `other` - The other hand to compare to
        ///
        /// # Returns
        /// The ordering of the comparison between the two hands
        /// `Ordering::Equal` if the hands are tied, `Ordering::Greater` if
        /// the first hand wins, `Ordering::Less` if the second hand wins.
        fn compare_two_pair(&self, other: &Hand) -> Ordering
        {
            let self_pairs = self.get_pair_values();
            let other_pairs = other.get_pair_values();

            // Compare higher pair first, then lower pair, then kicker
            match self_pairs[1].cmp(&other_pairs[1])
            {
                // lower pair
                Ordering::Equal => match self_pairs[0].cmp(&other_pairs[0])
                {
                    Ordering::Equal => self.compare_high_cards(other),
                    other_ordering => other_ordering,
                },
                other_ordering => other_ordering, // kicker
            }
        }

        /// Compare one pair with another
        ///
        /// # Arguments
        /// * `other` - The other hand to compare to
        ///
        /// # Returns
        /// The ordering of the comparison between the two hands
        /// `Ordering::Equal` if the hands are tied, `Ordering::Greater` if
        /// the first hand wins, `Ordering::Less` if the second hand wins.
        fn compare_one_pair(&self, other: &Hand) -> Ordering
        {
            // Compare the pair values.
            let self_pair = self.get_n_of_a_kind_value(2);
            let other_pair = other.get_n_of_a_kind_value(2);

            match self_pair.cmp(&other_pair)
            {
                Ordering::Equal => self.compare_high_cards(other),
                other_ordering => other_ordering,
            }
        }

        /// Compare the high cards of two hands
        ///
        /// # Arguments
        /// * `other` - The other hand to compare to
        ///
        /// # Returns
        /// The ordering of the comparison between the two hands
        /// `Ordering::Equal` if the hands are tied, `Ordering::Greater` if
        /// the first hand wins, `Ordering::Less` if the second hand wins.
        fn compare_high_cards(&self, other: &Hand) -> Ordering
        {
            let mut self_values: Vec<Value> = self.values().collect();
            let mut other_values: Vec<Value> = other.values().collect();

            self_values.sort_unstable_by(|a, b| b.cmp(a));
            other_values.sort_unstable_by(|a, b| b.cmp(a));

            // Just compare all high cards in descending order
            self_values.cmp(&other_values)
        }

        /// Get the value of the n-of-a-kind in the hand
        ///
        /// # Arguments
        /// * `n` - The number of a kind to get the value of
        ///
        /// # Returns
        /// The value of the n-of-a-kind in the hand
        fn get_n_of_a_kind_value(&self, n: usize) -> Value
        {
            let counts = self.value_counts();

            counts
                .iter()
                .find(|&(_value, &count)| count == n)
                .map(|(&value, _count)| value)
                .unwrap()
        }

        /// Get the values of pairs in the hand
        ///
        /// This is useful for comparing two pairs
        ///
        /// # Returns
        /// The counts of values that are pairs in the hand
        fn get_pair_values(&self) -> Vec<Value>
        {
            let counts = self.value_counts();

            let mut pairs: Vec<Value> = counts
                .iter()
                .filter(|&(_, &count)| count == 2)
                .map(|(&value, _)| value)
                .collect();

            pairs.sort_unstable();

            pairs
        }
    }

    impl PartialEq for Hand
    {
        fn eq(&self, other: &Self) -> bool
        {
            self.compare(other) == Ordering::Equal
        }
    }

    impl Eq for Hand
    {
    }

    impl Ord for Hand
    {
        fn cmp(&self, other: &Self) -> Ordering
        {
            self.compare(other)
        }
    }

    impl PartialOrd for Hand
    {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering>
        {
            Some(self.cmp(other))
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::poker::{Card, Hand, Suit, Value};

    #[test]
    fn test_card_new()
    {
        let card = Card::new("TH");
        assert_eq!(card.value, Value::Ten);
        assert_eq!(card.suit, Suit::Hearts);
    }

    #[test]
    fn test_card_new_all_values()
    {
        assert_eq!(Card::new("2S").value, Value::Two);
        assert_eq!(Card::new("3D").value, Value::Three);
        assert_eq!(Card::new("4C").value, Value::Four);
        assert_eq!(Card::new("5H").value, Value::Five);
        assert_eq!(Card::new("6S").value, Value::Six);
        assert_eq!(Card::new("7D").value, Value::Seven);
        assert_eq!(Card::new("8C").value, Value::Eight);
        assert_eq!(Card::new("9H").value, Value::Nine);
        assert_eq!(Card::new("JS").value, Value::Jack);
        assert_eq!(Card::new("QD").value, Value::Queen);
        assert_eq!(Card::new("KC").value, Value::King);
        assert_eq!(Card::new("AH").value, Value::Ace);
    }

    #[test]
    fn test_card_new_all_suits()
    {
        assert_eq!(Card::new("AH").suit, Suit::Hearts);
        assert_eq!(Card::new("AD").suit, Suit::Diamonds);
        assert_eq!(Card::new("AC").suit, Suit::Clubs);
        assert_eq!(Card::new("AS").suit, Suit::Spades);
    }

    #[test]
    fn test_hand_is_one_pair_true()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("9C"),
            Card::new("KS"),
            Card::new("AH"),
        ]);
        assert!(hand.is_one_pair());
    }

    #[test]
    fn test_hand_is_one_pair_false()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("3D"),
            Card::new("9C"),
            Card::new("KS"),
            Card::new("AH"),
        ]);
        assert!(!hand.is_one_pair());
    }

    #[test]
    fn test_hand_is_two_pair_true()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("9C"),
            Card::new("9S"),
            Card::new("AH"),
        ]);
        assert!(hand.is_two_pair());
    }

    #[test]
    fn test_hand_is_two_pair_false()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("9C"),
            Card::new("KS"),
            Card::new("AH"),
        ]);
        assert!(!hand.is_two_pair());
    }

    #[test]
    fn test_hand_is_three_of_a_kind_true()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("2C"),
            Card::new("KS"),
            Card::new("AH"),
        ]);
        assert!(hand.is_three_of_a_kind());
    }

    #[test]
    fn test_hand_is_three_of_a_kind_false()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("9C"),
            Card::new("KS"),
            Card::new("AH"),
        ]);
        assert!(!hand.is_three_of_a_kind());
    }

    #[test]
    fn test_hand_is_straight_true()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("3D"),
            Card::new("4C"),
            Card::new("5S"),
            Card::new("6H"),
        ]);
        assert!(hand.is_straight());
    }

    #[test]
    fn test_hand_is_straight_false()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("3D"),
            Card::new("4C"),
            Card::new("6S"),
            Card::new("7H"),
        ]);
        assert!(!hand.is_straight());
    }

    #[test]
    fn test_hand_is_flush_true()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("5H"),
            Card::new("9H"),
            Card::new("KH"),
            Card::new("AH"),
        ]);
        assert!(hand.is_flush());
    }

    #[test]
    fn test_hand_is_flush_false()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("5D"),
            Card::new("9H"),
            Card::new("KH"),
            Card::new("AH"),
        ]);
        assert!(!hand.is_flush());
    }

    #[test]
    fn test_hand_is_full_house_true()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("2C"),
            Card::new("KS"),
            Card::new("KH"),
        ]);
        assert!(hand.is_full_house());
    }

    #[test]
    fn test_hand_is_full_house_false()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("2C"),
            Card::new("KS"),
            Card::new("AH"),
        ]);
        assert!(!hand.is_full_house());
    }

    #[test]
    fn test_hand_is_four_of_a_kind_true()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("2C"),
            Card::new("2S"),
            Card::new("AH"),
        ]);
        assert!(hand.is_four_of_a_kind());
    }

    #[test]
    fn test_hand_is_four_of_a_kind_false()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("2C"),
            Card::new("KS"),
            Card::new("AH"),
        ]);
        assert!(!hand.is_four_of_a_kind());
    }

    #[test]
    fn test_hand_is_straight_flush_true()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("3H"),
            Card::new("4H"),
            Card::new("5H"),
            Card::new("6H"),
        ]);
        assert!(hand.is_straight_flush());
    }

    #[test]
    fn test_hand_is_straight_flush_false()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("3H"),
            Card::new("4H"),
            Card::new("5H"),
            Card::new("7H"),
        ]);
        assert!(!hand.is_straight_flush());
    }

    #[test]
    fn test_hand_is_royal_flush_true()
    {
        let hand = Hand::new([
            Card::new("TH"),
            Card::new("JH"),
            Card::new("QH"),
            Card::new("KH"),
            Card::new("AH"),
        ]);
        assert!(hand.is_royal_flush());
    }

    #[test]
    fn test_hand_is_royal_flush_false()
    {
        let hand = Hand::new([
            Card::new("9H"),
            Card::new("TH"),
            Card::new("JH"),
            Card::new("QH"),
            Card::new("KH"),
        ]);
        assert!(!hand.is_royal_flush());
    }

    #[test]
    fn test_hand_is_straight_ace_low()
    {
        let hand = Hand::new([
            Card::new("AH"),
            Card::new("2D"),
            Card::new("3C"),
            Card::new("4S"),
            Card::new("5H"),
        ]);
        assert!(hand.is_straight());
    }

    #[test]
    fn test_hand_is_straight_false_with_duplicates()
    {
        let hand = Hand::new([
            Card::new("2H"),
            Card::new("2D"),
            Card::new("3C"),
            Card::new("4S"),
            Card::new("5H"),
        ]);
        assert!(!hand.is_straight());
    }
}
