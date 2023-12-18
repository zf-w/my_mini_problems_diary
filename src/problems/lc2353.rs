//! ## Leetcode 2353. Design a Food Rating System
//! https://leetcode.com/problems/design-a-food-rating-system
//! - `Medium`; `Learned`; `2023-12-16`;
//! 
//! I guess using an updatable heap would be a better solution. Also, the cuisines should better be mapped into some integers during the construction process. The strings should be references, if possible, instead of cloning them every time.


use std::collections::{HashMap, BTreeSet};

#[derive(Clone)]
struct Info {
  food: String,
  rating: i32,
  cuisine: String
}

impl Info {
  pub fn new(food: String, rating: i32, cuisine: String) -> Self {
    Info {
      food,
      rating,
      cuisine
    }
  }
}

impl PartialEq for Info {
  fn eq(&self, other: &Self) -> bool {
    self.food == other.food && self.rating == other.rating
  }
}

impl Eq for Info {

}

impl PartialOrd for Info {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      if self.rating == other.rating {
          Some(self.food.cmp(&other.food).reverse())
      } else {
          Some(self.rating.cmp(&other.rating))
      }
  }
}

impl Ord for Info {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      self.partial_cmp(other).unwrap()
  }
}

struct FoodRatings {
  cuisines_map: HashMap<String, BTreeSet<Info>>,
  foods_map: HashMap<String, Info>
}

impl FoodRatings {

    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
      
      let mut cuisines_map: HashMap<String, BTreeSet<Info>> = HashMap::new();
      let mut foods_map: HashMap<String, Info> = HashMap::with_capacity(foods.len() * 2);

      let mut cuisines_i = cuisines.iter();
      let mut ratings_i = ratings.iter();

      for food in foods.iter() {
        let cuisine = cuisines_i.next().expect("should have enough cuisine");
        let rating = ratings_i.next().expect("should have enough ratings");
        let info = Info::new(food.clone(), rating.clone(), cuisine.clone());
        if let Some(some_foods) = cuisines_map.get_mut(cuisine) {
          some_foods.insert(Info::new(food.clone(), rating.clone(), cuisine.clone()));
        } else {
          let mut some_foods: BTreeSet<Info> = BTreeSet::new();
          some_foods.insert(Info::new(food.clone(), rating.clone(), cuisine.clone()));
          cuisines_map.insert(cuisine.clone(), some_foods);
        }
        foods_map.insert(food.clone(),info);
      }
      
      FoodRatings {
        cuisines_map,
        foods_map
      }
    }
    
    fn change_rating(&mut self, food: String, new_rating: i32) {
        let food_info: Info = self.foods_map.get(&food).expect("food should exist").clone();
        let curr_foods = 
          self.cuisines_map.get_mut(&food_info.cuisine).expect("Cuisine not exists");

        let new_info = Info::new(food.clone(), new_rating, food_info.cuisine.clone());
        self.foods_map.remove(&food);
        self.foods_map.insert(food.clone(), new_info.clone());
        curr_foods.remove(&food_info);
        curr_foods.insert(new_info);
    }
    
    fn highest_rated(&self, cuisine: String) -> String {
        let curr_foods = 
          self.cuisines_map.get(&cuisine).expect("Cuisine not exists");
        // for food in curr_foods.iter() {
        //   print!("({} {})", food.food, food.rating);
        // }
        // println!("");
        curr_foods.iter().rev().next().expect("should have food").food.clone()
    }
}