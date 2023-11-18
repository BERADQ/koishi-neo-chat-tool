import { log } from "console";
import { serialization, deserialization } from "./pkg";
import fs from "fs";

let a: { [key: string]: string }[] = [];
for (let i = 0; i < 1000; i++) {
  a.push({ role: "user", content: "hello" });
}
let start_time = Date.now();
fs.writeFileSync("./test.bin", serialization(a));
//fs.writeFileSync("./test.json", JSON.stringify(a), "utf-8");
log(Date.now() - start_time, "ms");
