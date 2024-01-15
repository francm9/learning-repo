import express from "express";
import morgan from "morgan";
import bodyParser from "body-parser";
import cors from "cors";

const app = express();

// Settings
app.set("port", process.env.PORT || 3000);
app.set("json spaces", 2);

app.use(bodyParser.json())

app.post("/", async(req, res) => {
    console.log(req.body);
    res.send(req.body);
});

// Arrancar el servidor
app.listen(app.get("port"), () => {
    console.log(`Server on port ${app.get("port")}`);
});
