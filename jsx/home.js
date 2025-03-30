import "../style/style.css";
function Home(){
    return(
        <>
        <h1>Home Page</h1>
        <p>Welcome to the home page.</p>
        </>
    );
}
export default Home;
ReactDOM.render(<Home />, document.getElementById("root"));
