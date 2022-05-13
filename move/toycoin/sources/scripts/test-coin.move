script {
    use 0x2::Coin;

    fun main() {
        let _coin = Coin::mint(100);
    }
}