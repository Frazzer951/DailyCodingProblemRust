// NOT DONE

/* HARD
Soundex [https://en.wikipedia.org/wiki/Soundex] is an algorithm used to
categorize phonetically, such that two names that sound alike but are spelled
differently have the same representation.

Soundex maps every name to a string consisting of one letter and three numbers,
like M460.

One version of the algorithm is as follows:

 1. Remove consecutive consonants with the same sound (for example, change ck ->
    c).
 2. Keep the first letter. The remaining steps only apply to the rest of the
    string.
 3. Remove all vowels, including y, w, and h.
 4. Replace all consonants with the following digits: * b, f, p, v → 1
     * c, g, j, k, q, s, x, z →
       2
     * d, t → 3
     * l → 4
     * m, n → 5
     * r → 6


 5. If you don't have three numbers yet, append zeros until you do. Keep the
    first three numbers.

Using this scheme, Jackson and Jaxen both map to J250.

Implement Soundex.
*/

//fn problem_604() -> i64 {
//    0
//}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_problem_604() {
//        assert_eq!(problem_604(), 1);
//    }
//}
