#include <iostream>
#include <string>
using namespace std;

int text_to_code(char c) {
    if(c == '_')
        return 0;
    if(c == '.')
        return 27;
    return c - 'a' + 1;
}

char code_to_text(int c) {
    if(c == 0)
        return '_';
    if(c == 27)
        return '.';
    return 'a' + c - 1;
}

int main() {
    ios::sync_with_stdio(false);
	cin.tie(0);

    int k;
    while(1) {
        char result[100] = {};
        cin >> k;
        if(k == 0) break;
        string message;
        cin >> message;
        for(int i = 0; i < message.size(); ++i) {
            int plaincode = (text_to_code(message[i]) + 28) + i;
            result[(k * i) % message.size()] = code_to_text(plaincode % 28);
        }
        for(int i = 0; i < message.size(); ++i) {
            cout << result[i];
        }
        cout << "\n" << flush;
    }

    return 0;
}