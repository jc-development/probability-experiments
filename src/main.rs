use std::collections::HashSet;

fn is_heads_or_tails(outcome: &&str) -> bool {
    ["Heads", "Tails"].contains(&outcome)
}

fn is_neither(outcome: &&str) -> bool {
    !is_heads_or_tails(outcome)
}

fn is_heads(outcome: &&str) -> bool {
    outcome == &"Heads"
}

fn is_tails(outcome: &&str) -> bool {
    outcome == &"Tails"
}

fn get_matching_event<'a, F>(
    event_condition: F,
    sample_space: &'a HashSet<&'a str>,
) -> HashSet<&'a str>
where
    F: FnMut(&&str) -> bool,
{
    sample_space
        .iter()
        .cloned()
        .filter(event_condition)
        .collect()
}

fn compute_probability<F>(event_condition: F, generic_sample_space: &HashSet<&str>) -> f32
where
    F: FnMut(&&str) -> bool,
{
    let event = get_matching_event(event_condition, generic_sample_space);
    event.len() as f32 / generic_sample_space.len() as f32
}

fn main() {
    let sample_space: HashSet<_> = ["Heads", "Tails"].iter().cloned().collect();

    let event_conditions = [
        ("is_heads_or_tails", is_heads_or_tails as fn(&&str) -> bool),
        ("is_heads", is_heads),
        ("is_tails", is_tails),
        ("is_neither", is_neither),
    ];

    for (name, event_condition) in &event_conditions {
        let event = get_matching_event(event_condition, &sample_space);
        println!("Event Condition: {}", name);
        println!("Event: {:?}\n", event);
    }

    for (name, event_condition) in &event_conditions {
        let prob = compute_probability(*event_condition, &sample_space);
        println!("Probability of event arising from '{}' is {}", name, prob);
    }
}
