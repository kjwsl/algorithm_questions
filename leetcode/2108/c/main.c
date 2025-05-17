#include <stdbool.h>
#include <string.h>

bool isPalindrome(char* word) {
    int i = 0, j = strlen(word) - 1;

    while (i < j) {
        if (word[i++] != word[j--]) {
            return false;
        }
    }
    return true;
}

char* firstPalindrome(char** words, int wordsSize) {
    for (int i = 0; i < wordsSize; i++) {
        if (isPalindrome(words[i])){
            return words[i];
        }
    }

    return "";
}



