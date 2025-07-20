#include <iostream>

int main() {
    int n, m;
    bool arr[1001] = {};
    int round = 1;
    while(true) {
        std::cin >> m >> n;
        if(m == 0 && n == 0) {
            break;
        }

        for(int i = 0; i <= m*n; ++i) {
            arr[i] = false;
        }

        for(int i = 0; i < n; ++i) {
            int temp;
            std::cin >> temp;
            arr[temp] = true;
        }

        int win_count = 0;
        int lose_count = 0;
        for(int i = n * m; i >= 1; --i) {
            if(arr[i]) {
                if(lose_count > 0) {
                    lose_count--;
                }
                else {
                    win_count++;
                }
            }
            else {
                lose_count++;
            }
        }

        std::cout << "Case " << round << ": " << win_count << "\n";
        round++;
    }

    return 0;
}