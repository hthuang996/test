import { beginCell } from "ton";

function initData() {
  const initialCounterValue = 17;
  return beginCell().storeUint(initialCounterValue, 64).endCell();
}