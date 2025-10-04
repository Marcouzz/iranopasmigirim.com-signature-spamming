import axios from "axios";
import crypto from "crypto";

const url = "https://iranopasmigirim.com/fa";
const baseAccept = "text/x-component";
const baseAcceptLanguage = "en-IR,en;q=0.9,fa-IR;q=0.8,fa;q=0.7,az-IR;q=0.6,az;q=0.5,en-GB;q=0.4,en-US;q=0.3";
const baseContentType = "text/plain;charset=UTF-8";
const referer = "https://iranopasmigirim.com/fa";

function randInt(min, max) {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

function genChromeVersion() {
  const major = randInt(100, 116);
  const build = randInt(4800, 5100);
  return `${major}.0.${build}.0`;
}

function genUserAgent() {
  const chrome = genChromeVersion();
  const osChoices = [
    ["Windows NT 10.0; Win64; x64", "Windows"],
    ["Macintosh; Intel Mac OS X 10_15_7", "macOS"],
    ["X11; Linux x86_64", "Linux"]
  ];
  const choice = osChoices[randInt(0, osChoices.length - 1)];
  const uaString = `Mozilla/5.0 (${choice[0]}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${chrome} Safari/537.36`;
  return [uaString, choice[1]];
}

function genSecChUa(chromeVersion) {
  const major = chromeVersion.split(".")[0];
  const labels = [
    `"Google Chrome";v="${major}"`,
    `"Not?A_Brand";v="8"`,
    `"Chromium";v="${major}"`
  ];
  labels.sort(() => Math.random() - 0.5);
  return labels.join(", ");
}

function genGaCookie() {
  const clientId = `${randInt(10000000, 99999999)}.${Math.floor(Date.now() / 1000)}`;
  return `GA1.1.${clientId}`;
}

function genGaDynamic() {
  const suffix = crypto.randomBytes(6).toString("hex").toUpperCase();
  const value = `GS2.1.s${Math.floor(Date.now()/1000)}$o1$g1$t${Math.floor(Date.now()/1000)}$j${randInt(1,100)}$l0$h0`;
  return [suffix, value];
}

let count = 10000000;
(async () => {
  for (let i = 0; i < count; i++) {
    const [uaString, uaPlatform] = genUserAgent();
    const chromeMajor = uaString.split("Chrome/")[1].split(".")[0];
    const secChUa = genSecChUa(uaString.split("Chrome/")[1]);
    const secChUaMobile = "?0";
    const secChUaPlatform = `"${uaPlatform}"`;

    const headers = {
      "accept": baseAccept,
      "accept-language": baseAcceptLanguage,
      "cache-control": "no-cache",
      "content-type": baseContentType,
      "next-action": crypto.randomBytes(16).toString("hex") + crypto.randomBytes(4).toString("hex"),
      "next-router-state-tree": "%5B%22%22%2C%7B%22children%22%3A%5B%5B%22lang%22%2C%22fa%22%2C%22d%22%5D%2C%7B%22children%22%3A%5B%22(used)%22%2C%7B%22children%22%3A%5B%22(main)%22%2C%7B%22children%22%3A%5B%22__PAGE__%22%2C%7B%7D%2C%22%2Ffa%22%2C%22refresh%22%5D%7D%5D%7D%5D%7D%5D%7D%2Cnull%2Cnull%2Ctrue%5D",
      "pragma": "no-cache",
      "priority": "u=1, i",
      "sec-ch-ua": secChUa,
      "sec-ch-ua-mobile": secChUaMobile,
      "sec-ch-ua-platform": secChUaPlatform,
      "sec-fetch-dest": "empty",
      "sec-fetch-mode": "cors",
      "sec-fetch-site": "same-origin",
      "x-deployment-id": "dpl_" + crypto.randomBytes(8).toString("hex"),
      "referer": referer
    };

    const ga = genGaCookie();
    const [gaSuffix, gaValue] = genGaDynamic();
    const nowMs = Date.now();
    headers["cookie"] = `NEXT_LOCALE=fa; ${ga}; _ga_${gaSuffix}=${gaValue}; munich_sign_count=1; munich_last_sign=${nowMs}`;

    const bodyPayload = [{
      name: "Chrome Desktop User",
      signatureType: "typed",
      signatureData: uaString + " " + crypto.randomBytes(16).toString("hex")
    }];

    try {
      const resp = await axios.post(url, bodyPayload, { headers, timeout: 15000 });
      console.log(`${i+1}/${count} -> ${resp.status} ${resp.statusText}`);
    } catch (e) {
      console.log(`${i+1}/${count} -> error: ${e.message}`);
    }
  }
})();
