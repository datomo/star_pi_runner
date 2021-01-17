function invoke(arg) {
    window.external.invoke(JSON.stringify(arg));
}

let test;

function startTest() {
    invoke({ cmd: 'log', text: "started correctly"});
    test();

}

function addCallback(func) {
    test = func
}
function init() {
    invoke({ cmd: 'init' });
}
function log() {
    var s = '';
    for (var i = 0; i < arguments.length; i++) {
        if (i !== 0) {
            s = s + ' ';
        }
        s = s + JSON.stringify(arguments[i]);
    }
    invoke({ cmd: 'log', text: s });
}
function addTask(name) {
    invoke({ cmd: 'addTask', name: name });
}
function clearDoneTasks() {
    invoke({ cmd: 'clearDoneTasks' });
}
function markTask(index, done) {
    invoke({ cmd: 'markTask', index: index, done: done });
}

export { init, log, addTask, clearDoneTasks, markTask, addCallback };