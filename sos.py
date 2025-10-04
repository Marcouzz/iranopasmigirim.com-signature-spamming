import requests
import time
import random
import uuid
import json

url = "https://iranopasmigirim.com/fa"
base_accept = "text/x-component"
base_accept_language = "en-IR,en;q=0.9,fa-IR;q=0.8,fa;q=0.7,az-IR;q=0.6,az;q=0.5,en-GB;q=0.4,en-US;q=0.3"
base_content_type = "text/plain;charset=UTF-8"
referer = "https://iranopasmigirim.com/fa"

def gen_chrome_version():
    major = random.randint(100, 116)
    minor = 0
    build = random.randint(4800, 5100)
    patch = 0
    return f"{major}.0.{build}.{patch}"

def gen_user_agent():
    chrome = gen_chrome_version()
    os_choice = random.choice([
        ("Windows NT 10.0; Win64; x64", "Windows"),
        ("Macintosh; Intel Mac OS X 10_15_7", "macOS"),
        ("X11; Linux x86_64", "Linux")
    ])
    return f"Mozilla/5.0 ({os_choice[0]}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{chrome} Safari/537.36", os_choice[1]

def gen_sec_ch_ua(chrome_version):
    labels = [
        f"\"Google Chrome\";v=\"{chrome_version.split('.')[0]}\"",
        "\"Not?A_Brand\";v=\"8\"",
        "\"Chromium\";v=\"{v}\"".replace("{v}", chrome_version.split('.')[0])
    ]
    random.shuffle(labels)
    return ", ".join(labels)

def gen_ga_cookie():
    client_id = f"{random.randint(10000000,99999999)}.{int(time.time())}"
    return f"GA1.1.{client_id}"

def gen_ga_dynamic():
    suffix = uuid.uuid4().hex[:12].upper()
    value = f"GS2.1.s{int(time.time())}$o1$g1$t{int(time.time())}$j{random.randint(1,100)}$l0$h0"
    return suffix, value

try:
    count = int(input("Enter number of requests: ").strip())
except Exception:
    count = 1

for i in range(count):
    ua_string, ua_platform = gen_user_agent()
    chrome_major = ua_string.split("Chrome/")[1].split(".")[0]
    sec_ch_ua = gen_sec_ch_ua(ua_string.split("Chrome/")[1])
    sec_ch_ua_mobile = "?0"
    sec_ch_ua_platform = f"\"{ua_platform}\""

    headers = {
        "accept": base_accept,
        "accept-language": base_accept_language,
        "cache-control": "no-cache",
        "content-type": base_content_type,
        "next-action": uuid.uuid4().hex + uuid.uuid4().hex[:8],
        "next-router-state-tree": "%5B%22%22%2C%7B%22children%22%3A%5B%5B%22lang%22%2C%22fa%22%2C%22d%22%5D%2C%7B%22children%22%3A%5B%22(used)%22%2C%7B%22children%22%3A%5B%22(main)%22%2C%7B%22children%22%3A%5B%22__PAGE__%22%2C%7B%7D%2C%22%2Ffa%22%2C%22refresh%22%5D%7D%5D%7D%5D%7D%5D%7D%2Cnull%2Cnull%2Ctrue%5D",
        "pragma": "no-cache",
        "priority": "u=1, i",
        "sec-ch-ua": sec_ch_ua,
        "sec-ch-ua-mobile": sec_ch_ua_mobile,
        "sec-ch-ua-platform": sec_ch_ua_platform,
        "sec-fetch-dest": "empty",
        "sec-fetch-mode": "cors",
        "sec-fetch-site": "same-origin",
        "x-deployment-id": "dpl_" + uuid.uuid4().hex[:16],
        "Referer": referer
    }

    ga = gen_ga_cookie()
    ga_suffix, ga_value = gen_ga_dynamic()
    cookie_header = "; ".join([
        "NEXT_LOCALE=fa",
        ga,
        f"_ga_{ga_suffix}={ga_value}"
    ])
    headers["cookie"] = cookie_header

    body_payload = [{
        "name": "Chrome Desktop User",
        "signatureType": "typed",
        "signatureData": ua_string + " " + uuid.uuid4().hex
    }]
    body = json.dumps(body_payload)

    try:
        resp = requests.post(url, headers=headers, data=body, timeout=15, stream=True)
        print(f"{i+1}/{count} -> {resp.status_code} {resp.reason}")
        resp.close()
    except Exception as e:
        print(f"{i+1}/{count} -> error: {e}")

    time.sleep(random.uniform(0.05, 0.18))
