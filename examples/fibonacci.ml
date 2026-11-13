# fibonacci.ml
# Kiraan jujukan Fibonacci secara rekursif dan iteratif

kerja fiboRekursif(n)
    jika n kurang sama 0
        pulang 0
    selesai
    jika n sama dengan 1
        pulang 1
    selesai
    pulang fiboRekursif(n tolak 1) tambah fiboRekursif(n tolak 2)
selesai

tunjuk "=== 10 NOMBOR PERTAMA FIBONACCI ==="
untuk k dari 0 hingga 9
    tunjuk "F(" + k + ") = " + fiboRekursif(k)
selesai

