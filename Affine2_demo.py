#Basically a demonstration of 'Affine2' (idk what its actually called), written in python to clarify whats going on
def encrypt(url, eq):
    url_chars = list(url)
    increment = 1
    for x in range(len(url_chars)):
        make_shift = False
        if url_chars[x].islower(): ascii_val, make_shift = 97, True
        elif url_chars[x].isupper(): ascii_val, make_shift = 65, True
        if make_shift:
            new_sum = 0
            for y in range(len(eq)): new_sum = new_sum + (eq[y] * (increment ** y)) #creates a polynomial that is binded to each alpha
            url_chars[x] = chr((ord(url_chars[x]) + (new_sum%26) - ascii_val) % 26 + ascii_val)
            increment += 1 #so our cypher is never constant
    return "".join(url_chars)

def decrypt(url, eq):
    url_chars = list(url)
    increment = 1
    for x in range(len(url_chars)):
        make_shift = False
        if url_chars[x].islower(): ascii_val, make_shift = 97, True
        elif url_chars[x].isupper(): ascii_val, make_shift = 65, True
        if make_shift:
            new_sum = 0
            for y in range(len(eq)): new_sum = new_sum + (eq[y] * (increment ** y)) #creates a polynomial that is binded to each alpha
            url_chars[x] = chr((ord(url_chars[x]) - (new_sum%26) - ascii_val) % 26 + ascii_val)
            increment += 1 #so our cypher is never constant
    return "".join(url_chars)

print(encrypt("https://www.youtube.com/watch?v=g97La0u55_g&ab_channel=KILTLE", [32,19,5]))
print(decrypt(encrypt("https://www.youtube.com/watch?v=g97La0u55_g&ab_channel=KILTLE", [32,19,5]),[32,19,5]))
print(decrypt("nhxra://sos.oqsurk.qso", [58,22,4])) #demo which was produced by a randomised dt.json

#I could of made them a single function but really I wasnt bothered lol
