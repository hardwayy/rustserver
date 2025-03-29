const path = require('path');

module.exports = {
    entry: './jsx/index.js', // Punto di ingresso del codice JavaScript
    output: {
        path: path.resolve(__dirname, 'public'), // Cartella di uscita (deve essere 'public' nel tuo caso)
        filename: 'bundle.js',  // Il nome del file di output
    },
    module: {
        rules: [
            {
                test: /\.js$/,  // Rileva i file con estensione .js
                exclude: /node_modules/,  // Non applicare a node_modules
                use: {
                    loader: 'babel-loader',  // Usa Babel per compilare il codice
                    options: {
                        presets: ['@babel/preset-env', '@babel/preset-react'],  // Usa preset per ES6 e React
                    },
                },
            },
        ],
    },
    resolve: {
        extensions: ['.js'], // Rileva solo file .js
    },
};
