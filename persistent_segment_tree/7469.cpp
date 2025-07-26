#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

struct Node {
    Node *left, *right;
    long long value;
    Node() {
        left = nullptr;
        right = nullptr;
        value = 0;
    }
};

Node *root[100010];
int numbers[100010];
int position[100010];

void build(Node *node, int start, int end) {
    if(start == end) {
        node->value = 0;
        return;
    }

    int mid = (start + end) / 2;
    node->left = new Node();
    node->right = new Node();
    build(node->left, start, mid);
    build(node-> right, mid + 1, end);
    node->value = node->left->value + node->right->value;
}

void add(Node *prev, Node *now, int start, int end, int x, int value) {
    if(start == end) {
        now->value = value;
        return;
    }
    int mid = (start + end) / 2;
    if(x <= mid) {
        now->left = new Node();
        now->right = prev->right;
        add(prev->left, now->left, start, mid, x, value);
    } else {
        now->left = prev->left;
        now->right = new Node();
        add(prev->right, now->right, mid + 1, end, x, value);
    }
    now->value = now->left->value + now->right->value;
}

int query(Node *left, Node *right, int start, int end, int k) {
    if(start == end) {
        return start;
    }
    int diff = right->left->value - left->left->value;
    int mid = (start + end) / 2;
    if(k <= diff) {
        return query(left->left, right->left, start, mid, k);
    } else {
        return query(left->right, right->right, mid + 1, end, k - diff);
    }
}

int main() {
    ios::sync_with_stdio(false);
	cin.tie(0);

    int n, m;
    cin >> n >> m;
    vector<pair<int, int>> sorted_numbers(n);

    for(int i=0; i<n; ++i) {
        cin >> numbers[i];
        sorted_numbers[i] = make_pair(numbers[i], i);
    }
    sort(sorted_numbers.begin(), sorted_numbers.end());

    for(int i=0; i<n; ++i) {
        position[sorted_numbers[i].second] = i;
    }

    root[0] = new Node();
    build(root[0], 0, n - 1);
    for(int i=0; i<n; ++i) {
        root[i + 1] = new Node();
        add(root[i], root[i + 1], 0, n - 1, position[i], 1);
    }

    for(int it=0; it<m; ++it) {
        int i, j, k;
        cin >> i >> j >> k;
        int index = query(root[i - 1], root[j], 0, n - 1, k);
        cout << sorted_numbers[index].first << "\n";
    }

    return 0;
}