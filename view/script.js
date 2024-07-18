const displayJsonCreate = (cpuUsageAsJson) => {
    cpuUsageAsJson.forEach(cpuUsage => {
        let div = document.createElement('div');
        div.classList.add('bar-container');
        div.innerHTML = `
            <div class="bar" style="width: ${cpuUsage}%"></div>
            <div class="label">${cpuUsage.toFixed(2)}%</div>
        `;

        document.body.appendChild(div);
    });
};

const displayJsonUpdate = (cpuUsageAsJson) => {
    let barContainers = document.body.children;

    if (barContainers.length != cpuUsageAsJson.length) {
        throw new Error(
            `Error: number of bar containers are unequal to number of CPU usage percentage (${barContainers.length} != ${cpuUsageAsJson.length})`
        );
    }

    let i = 0;
    for (let barContainer of barContainers) {
        let childrens = barContainer.children;

        for (let child of childrens) {
            if (child.className === 'bar') {
                child.style.width = `${cpuUsageAsJson[i]}%`;
            } else if (child.className === 'label') {
                child.innerText = `${cpuUsageAsJson[i].toFixed(2)}%`;
            } else {
                throw new Error('Error: unreachable');
            }
        }

        i++;
    }
}

let notDrawnOnce = true;

let url = new URL('/realtime/cpus', window.location.href);
url.protocol = url.protocol.replace('http', 'ws');

let ws = new WebSocket(url.href);
ws.onmessage = (event) => {
    let cpuUsages = JSON.parse(event.data);

    if (notDrawnOnce) {
        displayJsonCreate(cpuUsages);
        notDrawnOnce = false;
    } else {
        displayJsonUpdate(cpuUsages);
    }
}
