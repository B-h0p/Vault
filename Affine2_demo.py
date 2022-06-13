#Basically a demonstration of 'Affine2' (idk what its actually called), written in python to clarify whats going on
def encrypt(url, eq):
    url_chars = list(url)
    increment = 1
    for x in range(len(url_chars)):
        if url_chars[x].islower():
            new_sum = 0
            for y in range(len(eq)): new_sum = new_sum + (eq[y] * (increment ** y)) #creates a polynomial that is binded to each alpha
            url_chars[x] = chr((ord(url_chars[x]) + (new_sum%26) - 97) % 26 + 97)
            increment += 1 #so our cypher is never constant
        elif url_chars[x].isupper():
            new_sum = 0
            for y in range(len(eq)): new_sum = new_sum + (eq[y] * (increment ** y))
            url_chars[x] =  chr((ord(url_chars[x]) + (new_sum%26) - 65) % 26 + 65)
            increment += 1
    return "".join(url_chars)

def decrypt(url, eq):
    url_chars = list(url)
    increment = 1
    for x in range(len(url_chars)):
        if url_chars[x].islower():
            new_sum = 0
            for y in range(len(eq)): new_sum = new_sum + (eq[y] * (increment ** y))
            url_chars[x] = chr((ord(url_chars[x]) - (new_sum%26) - 97) % 26 + 97) #(new_sum%26) hastens the calculation without changing the functions pattern
            increment += 1
        elif url_chars[x].isupper():
            new_sum = 0
            for y in range(len(eq)): new_sum = new_sum + (eq[y] * (increment ** y))    
            url_chars[x] =  chr((ord(url_chars[x]) - (new_sum%26) - 65) % 26 + 65)
            increment += 1 
    return "".join(url_chars)

print(encrypt("https://www.youtube.com/watch?v=g97La0u55_g&ab_channel=KILTLE", [32,19,5]))
print(decrypt("lfxvk://kqg.iiilafq.gue/kudmb?j=y97Re0g55_k&gt_qbkxhsd=QMXXRW",[32,19,5]))
print(decrypt("nhxra://sos.oqsurk.qso", [58,22,4])) #demo which was produced in sample by a randomised dt.json

#I could of made them a single function but really I wasnt bothered lol
