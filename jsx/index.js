import Test from './components/test.js';
import Component from './components/component.js';
import "../style/style.css";


function Index() {
    return (
        <>
            <div className="flex h-screen items-center justify-center bg-gray-800 text-white">
                <div className="text-center">
                    <h1>Test server</h1>
                    <div className="flex items-center justify-center">
                        <p>This local server is hosted in Rust and the pages are written in JSX</p>
                    </div>
                    <Test/>
                    <Component/>
                </div>
            </div>

        </>
    );
}

ReactDOM.render(<Index />, document.getElementById("root"));
