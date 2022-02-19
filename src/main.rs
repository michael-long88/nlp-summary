use regex::Regex;
use std::{fs, collections::HashMap};

fn main() {
    let contents = fs::read_to_string("test.txt").expect("Something went wrong reading the file.");

    let stop_words = vec!["i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "her", "hers", "herself", "it", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", "who", "whom", "this", "that", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "no", "nor", "not", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "don", "should", "now"];
    let separator = Regex::new(r#" *[\.\?!]['"\)\]]* *"#).expect("Invalid regex");

    let sentences: Vec<_> = separator.split(&contents).into_iter().collect();
    let clean_text = contents.to_lowercase();
    let word_tokenize: Vec<_> = clean_text.split(" ").collect();

    let mut word2count = get_word_scores(word_tokenize, stop_words);

    let sent2score = get_sentence_scores(sentences, &word2count);

    let mut max_val: usize = 0;
    match word2count.values().max() {
        Some(val) => max_val = *val,
        None => ()
    }

    // created a weighted histogram
    for (_, val) in word2count.iter_mut() {
        *val /= &max_val;
    }

    let summary_sentences = get_summary(sent2score);
    println!("{}", summary_sentences);
}

/// Score each word based on number of occurances
fn get_word_scores(word_tokenize: Vec<&str>, stop_words: Vec<&str>) -> HashMap<String, usize> {
    let mut word2count: HashMap<String, usize> = HashMap::new();
    for word in word_tokenize {
        if !stop_words.contains(&word) {
            let count = word2count.entry(word.to_string()).or_insert(0);
            *count += 1;
        }
    }
    word2count
}

/// Score each sentence based on word scores
fn get_sentence_scores(sentences: Vec<&str>, word2count: &HashMap<String, usize>) -> HashMap<String, usize> {
    let mut sent2score: HashMap<String, usize> = HashMap::new();
    for sentence in sentences {
        let words: Vec<_> = sentence.split(" ").into_iter().collect();
        for word in &words {
            if word2count.contains_key(*word) {
                // only take the sentences that have a minimum of 9 words and a max of 28
                if words.len() < 28 && words.len() > 9 {
                    if let Some(word_val) = word2count.get(*word) {
                        let count = sent2score.entry(sentence.to_string()).or_insert(0);
                        *count += word_val;
                    }
                }
            }
        }
    }
    sent2score
}

/// Creates a 3 sentence summary from sentence scores
fn get_summary(sentence_scores: HashMap<String, usize>) -> String {
    let mut summary_sentences: Vec<&str> = Vec::new();
    let mut hash_vec: Vec<(&String, &usize)> = sentence_scores.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    let mut range = 3;
    if hash_vec.len() < range {
        range = hash_vec.len();
    }
    for i in 0..range {
        summary_sentences.push(hash_vec[i].0);
    }
    summary_sentences.join(". ").replace("\n", "")
}