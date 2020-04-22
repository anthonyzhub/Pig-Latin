# Pig-Latin
Rust's pig-latin challenge for new learners.

I wrote this program to demonstrate my understanding of Rust. In chapter 8, I learned the different types of common collections and how they are constructed in this language. Strings, for instance, are composed of vectors of type u8 in Rust, which caught me by surprised. The author did an excellent job of talking about the string's structure, in comparison to other computer science books.

At the end of chapter 8, the auther left a few challenges for the reader to solve. I like to solve challenges because it sharpens my skills and puts my knowledge to the test. One of the challenges were to take a string and translate it to pig latin. I wrote the rules below. 

Enjoy!

**Here are the rules**:

1. If the word starts with a vowel, append "-hay" to the string.
  Ex. Electricity -> electricity-hay
  
2. If the word starts with a consonant, pop the 1st letter and append it to -{1st letter}ay, then append the new string to the current word.
  Ex. Horse -> orse-hay
