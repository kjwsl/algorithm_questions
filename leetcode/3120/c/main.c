#define ALPHABET_COUNT 26

bool is_lowercase(char ch) {
    return ch >= 'a' && ch <= 'z';
}

bool is_uppercase(char ch) {
    return ch >= 'A' && ch <= 'Z';
}

int numberOfSpecialChars(char* word) {
    int cnt = 0;
    int n = strlen(word);
    int ch_map[ALPHABET_COUNT * 2] = {0};

    for (int i = 0; i < n; i++) {
        char ch = word[i];

        int idx = -1;

        if (is_lowercase(ch)) {
            idx = ch - 'a';
        } else if (is_uppercase(ch)) {
            idx = ch - 'A' + ALPHABET_COUNT;
        } else {
            printf("WTF????");
            exit(1);
        }

        ch_map[idx]++;
    }

    for (int i = 0; i < ALPHABET_COUNT; i++) {
        if (ch_map[i] > 0 && ch_map[i + ALPHABET_COUNT] > 0) {
            cnt++;
        }
    }

    return cnt;
}
