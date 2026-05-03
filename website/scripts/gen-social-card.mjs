import puppeteer from 'puppeteer';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __dirname = dirname(fileURLToPath(import.meta.url));
const htmlPath = join(__dirname, 'social-card.html');
const outPath = join(__dirname, '../static/img/social-card.png');

const browser = await puppeteer.launch({ headless: 'new' });
const page = await browser.newPage();
await page.setViewport({ width: 1200, height: 630, deviceScaleFactor: 2 });
await page.goto('file://' + htmlPath);
await page.waitForNetworkIdle({ idleTime: 500 }).catch(() => {});
await page.screenshot({ path: outPath, type: 'png', clip: { x: 0, y: 0, width: 1200, height: 630 } });
await browser.close();
console.log('Saved:', outPath);
