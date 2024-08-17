with open('2016\\Input\\2016-07.txt') as input:
    ip_adresses = []
    for line in input:
        ip_adresses.append(line.strip())

def check_ip_adress(ip_adress, part):
    inside_brackets = False
    valid = False
    found_ssl = []

    end = 3 if part else 2

    for i in range(len(ip_adress) - end):
        if ip_adress[i] == '[':
            inside_brackets = True
            continue
        elif ip_adress[i] == ']':
            inside_brackets = False
            continue
        else:
            if part:
                c1, c2, c3, c4 = ip_adress[i], ip_adress[i + 1], ip_adress[i + 2], ip_adress[i + 3]
                if c1 == c4 and c2 == c3 and c1 != c3:
                    if inside_brackets:
                        return False
                    else:
                        valid = True
            else:
                c1, c2, c3 = ip_adress[i], ip_adress[i + 1], ip_adress[i + 2]
                if c1 == c3 and c2 != c1:
                    found_ssl.append((c1 + c2 + c3, inside_brackets))
                    if (c2 + c1 + c2, not inside_brackets) in found_ssl:
                        return True
    
    return valid

def count_valid_ip_adresses(ip_adress_list, part):
    count = 0

    for ip in ip_adress_list:
        count += 1 if check_ip_adress(ip, part) else 0

    return count

part_one = count_valid_ip_adresses(ip_adresses, True)
part_two = count_valid_ip_adresses(ip_adresses, False)

print('--- Day 7: Internet Protocol Version 7 ---')
print(f' -  Part one solution: {part_one}.')
print(f' -  Part two solution: {part_two}.')