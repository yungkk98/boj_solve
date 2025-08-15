#include <iostream>
using namespace std;

constexpr int n = 524288;

struct Node {
    int count;
    Node *left, *right;
    Node() {
        count = 0;
        left = right = nullptr;
    }
};

Node *root[500001];

void build(Node *node, int start, int end) {
    if(start == end) {
        return;
    }

    node->left = new Node();
    node->right = new Node();
    build(node->left, start, (start + end) / 2);
    build(node->right, (start + end) / 2 + 1, end);
    return;
}

void add(Node *prev, Node *node, int start, int end, unsigned int index, int depth) {
    if(start == end) {
        node->count = prev->count + 1;
        return;
    }
    unsigned int now_bit = (index >> (18 - depth)) & 1;
    if(now_bit == 0) {
        node->left = new Node();
        node->right = prev->right;
        add(prev->left, node->left, start, (start + end) / 2, index, depth + 1);
    } else {
        node->right = new Node();
        node->left = prev->left;
        add(prev->right, node->right, (start + end) / 2 + 1, end, index, depth + 1);
    }

    node->count = node->left->count + node->right->count;
}

int query_xor(Node *left, Node *right, int start, int end, unsigned int x, int depth) {
    if(start == end) {
        return start;
    }

    unsigned int now_bit = (x >> (18 - depth)) & 1;
    if(now_bit == 1) {
        if(right->left->count - left->left->count > 0) {
            return query_xor(left->left, right->left, start, (start + end) / 2, x, depth + 1);
        }
        else {
            return query_xor(left->right, right->right, (start + end) / 2 + 1, end, x, depth + 1);
        }
    } else {
        if(right->right->count - left->right->count > 0) {
            return query_xor(left->right, right->right, (start + end) / 2 + 1, end, x, depth + 1);
        }
        else {
            return query_xor(left->left, right->left, start, (start + end) / 2, x, depth + 1);
        }
    }
}

int query_count(Node *left, Node *right, int start, int end, int x) {
    if(start == end) {
        return right->count - left->count;
    }

    int mid = (start + end) / 2;
    if(x > mid) {
        return (right->left->count - left->left->count) + query_count(left->right, right->right, mid + 1, end, x);
    } else {
        return query_count(left->left, right->left, start, mid, x);
    }
}

int query_kth(Node *left, Node *right, int start, int end, int k) {
    if(start == end) {
        return start;
    }

    int mid = (start + end) / 2;
    int left_count = right->left->count - left->left->count;
    if(k <= left_count) {
        return query_kth(left->left, right->left, start, mid, k);
    } else {
        return query_kth(left->right, right->right, mid + 1, end, k - left_count);
    }
}

int main() {
    ios::sync_with_stdio(false);
	cin.tie(0);

    int cur_root = 0;
    root[cur_root] = new Node();
    build(root[cur_root], 0, n - 1);

    int m;
    cin >> m;
    for(int i = 0; i < m; ++i) {
        int t, l, r, x, k;
        cin >> t;
        switch(t) {
            case 1:
                cin >> x;
                root[cur_root + 1] = new Node();
                add(root[cur_root], root[cur_root + 1], 0, n - 1, x, 0);
                cur_root++;
                break;
            case 2:
                cin >> l >> r >> x;
                cout << query_xor(root[l - 1], root[r], 0, n - 1, x, 0) << "\n";
                break;
            case 3:
                cin >> k;
                cur_root -= k;
                break;
            case 4:
                cin >> l >> r >> x;
                cout << query_count(root[l - 1], root[r], 0, n - 1, x) << "\n";
                break;
            case 5:
                cin >> l >> r >> k;
                cout << query_kth(root[l - 1], root[r], 0, n - 1, k) << "\n";
                break;
        }
    }

    return 0;
}