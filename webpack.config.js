const path = require("path");

module.exports = {
    entry: "./jsx/index.js", // Punto di ingresso del codice JavaScript
    output: {
        path: path.resolve(__dirname, "public"), // Cartella di uscita
        filename: "bundle.js", // bundle per index, compiledhome per home
    },
    module: {
        rules: [
            {
                test: /\.css$/, // Rileva i file con estensione .css
                use: [
                    "style-loader", // Inserisce il CSS nel DOM
                    "css-loader", // Interpreta le @import e le url()
                    "postcss-loader", // Applica le trasformazioni PostCSS
                ],
            },
            {
                test: /\.js$/, // Rileva i file con estensione .js
                exclude: /node_modules/, // Esclude la cartella node_modules
                use: {
                    loader: "babel-loader", // Usa Babel per compilare il codice
                    options: {
                        presets: ["@babel/preset-env", "@babel/preset-react"], // Preset per ES6 e React
                    },
                },
            },
        ],
    },
    resolve: {
        extensions: [".js"], // Rileva solo file .js
    },
};
