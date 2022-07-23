/* MEDIUM
Given an unordered list of flights taken by someone, each represented as
(origin, destination) pairs, and a starting airport, compute the person's
itinerary. If no such itinerary exists, return null. If there are multiple
possible itineraries, return the lexicographically smallest one. All flights
must be used in the itinerary.

For example, given the list of flights [('SFO', 'HKO'), ('YYZ', 'SFO'), ('YUL',
'YYZ'), ('HKO', 'ORD')] and starting airport 'YUL', you should return the list
['YUL', 'YYZ', 'SFO', 'HKO', 'ORD'].

Given the list of flights [('SFO', 'COM'), ('COM', 'YYZ')] and starting airport
'COM', you should return null.

Given the list of flights [('A', 'B'), ('A', 'C'), ('B', 'C'), ('C', 'A')] and
starting airport 'A', you should return the list ['A', 'B', 'C', 'A', 'C'] even
though ['A', 'C', 'A', 'B', 'C'] is also a valid itinerary. However, the first
one is lexicographically smaller.
*/

fn problem_041(flights: Vec<(String, String)>, starting_airport: String) -> Option<Vec<String>> {
    let mut itinerary = vec![starting_airport.clone()];

    if flights.is_empty() {
        Some(itinerary)
    } else {
        let options: Vec<_> = flights.iter().filter(|f| f.0 == starting_airport).collect();
        if options.is_empty() {
            return None;
        }

        //println!("From: {:#?}", current);
        //println!("Options: {:#?}", options);

        let mut itineraries = vec![];
        for (_, to) in options {
            let index = flights
                .iter()
                .position(|x| *x == (starting_airport.clone(), to.clone()))
                .unwrap();
            let mut new_flights = flights.clone();
            new_flights.remove(index);
            let possible_itinerary = problem_041(new_flights, to.clone());
            if let Some(possible_itinerary) = possible_itinerary {
                itineraries.push(possible_itinerary);
            }
        }

        if itineraries.is_empty() {
            None
        } else {
            itineraries.sort();
            itinerary.extend(itineraries[0].clone());
            Some(itinerary)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_041_1() {
        let result = problem_041(
            vec![
                ("SFO".to_string(), "HKO".to_string()),
                ("YYZ".to_string(), "SFO".to_string()),
                ("YUL".to_string(), "YYZ".to_string()),
                ("HKO".to_string(), "ORD".to_string()),
            ],
            "YUL".to_string(),
        );
        let expected = Some(vec![
            "YUL".to_string(),
            "YYZ".to_string(),
            "SFO".to_string(),
            "HKO".to_string(),
            "ORD".to_string(),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_problem_041_2() {
        let result = problem_041(
            vec![
                ("SFO".to_string(), "COM".to_string()),
                ("COM".to_string(), "YYZ".to_string()),
            ],
            "COM".to_string(),
        );
        let expected = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_problem_041_3() {
        let result = problem_041(
            vec![
                ("A".to_string(), "B".to_string()),
                ("A".to_string(), "C".to_string()),
                ("B".to_string(), "C".to_string()),
                ("C".to_string(), "A".to_string()),
            ],
            "A".to_string(),
        );
        let expected = Some(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "A".to_string(),
            "C".to_string(),
        ]);
        assert_eq!(result, expected);
    }
}
