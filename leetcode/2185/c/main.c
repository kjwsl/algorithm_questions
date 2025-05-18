int prefixCount(char** words, int wordsSize, char* pref) {
    int cnt = 0;
    int pref_size = strlen(pref);
    for (int i = 0; i < wordsSize; i++) {
        int found = true;
        char* word = words[i];
        int word_size = strlen(word);
        if (word_size < pref_size) { continue; }
        for (int j = 0; j < pref_size; j++) {
            if (word[j] != pref[j]) {
                found = false;
                break;
            }
        }
        if (found) {
            cnt++;
        }
    }

    return cnt;
}
