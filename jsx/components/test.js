function Test() {

    function clicked(){
        alert('clicked');
    }

    return (
        <>
            <div>
                <button id={"testBut"} onClick={clicked} className="rounded-lg">Click me!</button>
            </div>

        </>
    );
}
export default Test;
