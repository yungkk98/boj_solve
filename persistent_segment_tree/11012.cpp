#include <iostream>
#include <vector>
using namespace std;

struct Node {
    Node *left, *right;
    int value;
    Node() {
        left = nullptr;
        right = nullptr;
        value = 0;
    }
};

Node *root[100010];
vector<int> house[100010];

void build(Node *node, int start, int end) {
    if(start == end) {
        return;
    }

    int mid = (start + end) / 2;
    node->left = new Node();
    node->right = new Node();
    build(node->left, start, mid);
    build(node->right, mid + 1, end);
    node->value = node->left->value + node->right->value;
}

void add(Node *prev, Node *now, int start, int end, int x, int value) {
    if(start == end) {
        if(now->value == 0) {
            now->value = prev->value + value;
        } else {
            now->value += value;
        }
        return;
    }
    int mid = (start + end) / 2;
    if(x <= mid) {
        if(now->left == nullptr || now->left == prev->left) {
            now->left = new Node();
        }
        if(now->right == nullptr) {
            now->right = prev->right;
        }
        add(prev->left, now->left, start, mid, x, value);
    } else {
        if(now->left == nullptr) {
            now->left = prev->left;
        }
        if(now->right == nullptr || now->right == prev->right) {
            now->right = new Node();
        }
        add(prev->right, now->right, mid + 1, end, x, value);
    }
    now->value = now->left->value + now->right->value;
}

int query(Node *left, Node *right, int start, int end, int l, int r) {
    if (r < start || l > end) {
        return 0;
    }
    if(l <= start && end <= r) {
        return right->value - left->value;
    }

    int mid = (start + end) / 2;
    return query(left->left, right->left, start, mid, l, r) + query(left->right, right->right, mid + 1, end, l, r);
}

int main() {
    ios::sync_with_stdio(false);
	cin.tie(0);

    int t;
    cin >> t;
    for(int c=0; c<t; ++c) {
        if(c != 0) {
            for(int i=0; i<100010; ++i) {
                house[i].clear();
            }
        }

        int n, m;
        cin >> n >> m;
        root[0] = new Node();
        build(root[0], 1, 100001);
        for(int i=0; i<n; ++i) {
            int x, y;
            cin >> x >> y;
            house[y + 1].push_back(x + 1);
        }
        for(int i=1; i<100010; ++i) {
            if(house[i].size() == 0) {
                root[i] = root[i - 1];
            } else {
                root[i] = new Node();
                for(auto x : house[i]) {
                    add(root[i - 1], root[i], 1, 100001, x, 1);
                }
            }
        }

        int result = 0;
        for(int i=0; i<m; ++i) {
            int l, r, b, t;
            cin >> l >> r >> b >> t;
            int v = query(root[b], root[t + 1], 1, 100001, l + 1, r + 1);
            result += v;
        }

        cout << result << "\n";
    }

    return 0;
}