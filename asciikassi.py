def draw_ASCII_box(n):
    print(f"+{'-'*n}+")
    for _ in range(n):
        print(f"|{' '*n}|")
    print(f"+{'-'*n}+")

draw_ASCII_box(int(input()))
