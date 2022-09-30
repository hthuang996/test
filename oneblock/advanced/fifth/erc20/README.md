# ERC20
This is an ERC20 contract in `ink`.

## Specification
This ERC20 contract is based on the example code in [ink](https://github.com/paritytech/ink/tree/master/examples/erc20) and I made several improvements to it.

- Remove the limit to the total tokens, as well as the initial tokens for the caller.
- Add the method `mint`
- Add the method `burn`
- Add an owner for the contract.

## Snapshot
### Test

![image](https://user-images.githubusercontent.com/83948501/192823114-fa51b856-9d77-434d-967c-0048efb3b9f6.png)

### Mint to Ferdie

![图片](https://user-images.githubusercontent.com/83948501/193164141-2931da25-1481-46b5-a90a-036d187f8ac9.png)

![图片](https://user-images.githubusercontent.com/83948501/193164183-dc5ccedb-1d16-4816-a7d2-750d230a147e.png)

![图片](https://user-images.githubusercontent.com/83948501/193164227-874cf381-0533-44ee-8d72-9f456412d1ea.png)

### Transfer to Bob

![图片](https://user-images.githubusercontent.com/83948501/193164382-b833dcaf-559f-40ca-bdfa-48194acb66b5.png)

![图片](https://user-images.githubusercontent.com/83948501/193164420-0a247704-8708-4459-b571-d5d95acb4aed.png)

![图片](https://user-images.githubusercontent.com/83948501/193164457-4eeac258-a320-49be-821f-5f730a55b7f4.png)

### Ferdie approves for Bob

![图片](https://user-images.githubusercontent.com/83948501/193164536-d08936c1-a2ee-4d36-8ada-e6964499f833.png)

![图片](https://user-images.githubusercontent.com/83948501/193164561-335b818d-1ce9-434e-aa74-0247103b6725.png)

### Bob transfers token from Ferdie to Alick

![图片](https://user-images.githubusercontent.com/83948501/193164634-a50fa465-bdbd-4684-9292-5a1506db6754.png)

![图片](https://user-images.githubusercontent.com/83948501/193164706-8e4f651d-ee41-48f8-9b4b-d1b3456c6010.png)

![图片](https://user-images.githubusercontent.com/83948501/193164927-b0a212c9-aaf7-480e-ac1f-0f5dc98142ce.png)

### Burn from Bob

![图片](https://user-images.githubusercontent.com/83948501/193164867-727efa1b-0fec-4814-a448-0a1bc5bb77ae.png)

![图片](https://user-images.githubusercontent.com/83948501/193164967-da7b3c20-f29c-40d1-9bdc-18f031b2aea6.png)

### Burn or mint will fail if the caller not the owner

![图片](https://user-images.githubusercontent.com/83948501/193165025-25e9da9d-4943-4eff-ab7e-3a80b7b7ecf7.png)

![图片](https://user-images.githubusercontent.com/83948501/193165054-3191c237-5788-4bb6-b4a6-48d451dd48ef.png)
