#include <iostream>
#include <string>
using namespace std;

int main() {
    ios::sync_with_stdio(false);
	cin.tie(0);

    int n;
    string temp;
    cin >> n;
    std::getline(std::cin, temp);
    for(int i=1; i<=n; ++i) {
        std::getline(std::cin, temp);
        cout << i << ". " << temp << "\n";
    }

    return 0;
}