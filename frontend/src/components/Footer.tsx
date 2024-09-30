export default function Footer(){
    return(
        <footer className="footer footer-center bg-base-300 text-base-content py-4 bottom-0">
            <p>
                Copyright © {new Date().getFullYear()} - All right reserved by unitix <br />
                <span className="font-bold">This website is not produced, endorsed, supported, or affiliated with Nintendo or The Pokémon Company.</span>
            </p>
        </footer>
    )
}