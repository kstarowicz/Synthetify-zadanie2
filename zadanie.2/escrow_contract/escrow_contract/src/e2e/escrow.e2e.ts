import { CasperClient, CLValue } from 'casper-js-sdk';

const client = new CasperClient('http://localhost:7777');

async function testSwap() {
    const deployHash = await client.putDeploy({
        // Twoje dane deployu
        session: {
            // ...
            "Swap": {
                sender: "adres_konta_nadawcy",
                recipient: "adres_konta_odbiorcy",
                amount: 1000, // ilość tokenów do wymiany
            }
        }
    });

    const deployResult = await client.getDeployInfo(deployHash);
    if (deployResult.execution_results[0].result.success) {
        console.log("Test swap zakończony sukcesem");
    } else {
        console.error("Test swap nie powiódł się");
    }
}

testSwap().catch(error => {
    console.error(error);
    process.exit(1);
});