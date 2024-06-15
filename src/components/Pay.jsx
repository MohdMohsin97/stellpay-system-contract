import React from 'react'
import StellPay from '../contracts/stellpay.ts'

const Pay = () => {

	const handleClick = async () => {
		console.log("Hello World")
		const {result} = await stellPay.pay_amount();
		console.log(result);
	}

  return (
    <div>
		<label for="Amount">Amount </label>
		<input type="text" id="fname" name="fname" />
		<input type="submit" onclick={handleClick} value="Submit" />
	  </div>  
  )
}

export default Pay;