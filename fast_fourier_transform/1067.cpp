#include <cstdio>
#include <complex>
#include <vector>
#include <cmath>
#include <algorithm>
using namespace std;

const double pi = acos(-1);

void FFT(vector<complex<double>> &f, complex<double> w) {
    int n = f.size();
    if(n == 1) {
        return;
    }
    vector<complex<double>> even(n / 2), odd(n / 2);
    for(int i=0; i<n; i++) {
        if (i % 2 == 1) {
            odd[i / 2] = f[i];
        } else {
            even[i / 2] = f[i];
        }
    }
    FFT(even, w * w);
    FFT(odd, w * w);
    complex<double> wp(1, 0);
    for(int i=0; i<n/2; i++) {
        f[i] = even[i] + wp * odd[i];
        f[i + n / 2] = even[i] - wp * odd[i];
        wp = wp * w;
    }
}

vector<complex<double>> mul(vector<complex<double>> a, vector<complex<double>> b) {
    int n = 1;
    while(n <= a.size() || n <= b.size()) {
        n *= 2;
    }
    n *= 2;
    a.resize(n);
    b.resize(n);
    vector<complex<double>> c(n);

    complex<double> w(cos(2 * pi / n), sin(2 * pi / n));
    FFT(a, w);
    FFT(b, w);

    for(int i=0; i<n; i++) {
        c[i] = a[i] * b[i];
    }
    FFT(c, complex<double>(w.real(), -w.imag()));
    for(int i=0; i<n; i++) {
        c[i] /= complex<double>(n, 0);
    }
    return c;
}

int main() {
    int n;
    scanf("%d", &n);
    vector<complex<double>> a(n * 2), b(n);
    for(int i=0; i<n; i++) {
        double temp;
        scanf("%lf", &temp);
        a[i] = complex<double>(temp, 0);
        a[i + n] = a[i];
    }
    for(int i=1; i<=n; i++) {
        double temp;
        scanf("%lf", &temp);
        b[n - i] = complex<double>(temp, 0);
    }

    vector<complex<double>> c = mul(a, b);
    unsigned long long result = 0; 
    for(auto it : c) {
        result = max(result, (unsigned long long )round(it.real()));
    }
    printf("%llu", result);

    return 0;
}