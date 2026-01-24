const loginSection = document.getElementById("login-section");
const twoFASection = document.getElementById("2fa-section");
const signupSection = document.getElementById("signup-section");

const signupLink = document.getElementById("signup-link");
const twoFALoginLink = document.getElementById("2fa-login-link");
const signupLoginLink = document.getElementById("signup-login-link");

signupLink.addEventListener("click", (e) => {
    e.preventDefault();

    loginSection.style.display = "none";
    twoFASection.style.display = "none";
    signupSection.style.display = "block";
});

twoFALoginLink.addEventListener("click", (e) => {
    e.preventDefault();

    loginSection.style.display = "block";
    twoFASection.style.display = "none";
    signupSection.style.display = "none";
});

signupLoginLink.addEventListener("click", (e) => {
    e.preventDefault();

    loginSection.style.display = "block";
    twoFASection.style.display = "none";
    signupSection.style.display = "none";
});

// -----------------------------------------------------

const loginForm = document.getElementById("login-form");
const loginButton = document.getElementById("login-form-submit");
const loginErrAlter = document.getElementById("login-err-alert");

loginButton.addEventListener("click", (e) => {
    e.preventDefault();

    const email = loginForm.email.value;
    const password = loginForm.password.value;

    fetch("/login", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
    }).then((response) => {
        if (response.status === 206) {
            twoFAForm.email.value = email;
            response.json().then(({ loginAttemptId }) => {
                twoFAForm.login_attempt_id.value = loginAttemptId;
            });

            loginForm.email.value = "";
            loginForm.password.value = "";

            loginSection.style.display = "none";
            twoFASection.style.display = "block";
            signupSection.style.display = "none";
            loginErrAlter.style.display = "none";
        } else if (response.status === 200) {
            loginForm.email.value = "";
            loginForm.password.value = "";
            loginErrAlter.style.display = "none";
            alert("You have successfully logged in.");
        } else {
            response.json().then(({ error }) => {
                if (error !== undefined && error !== null && error !== "") {
                    loginErrAlter.innerHTML = `<span><strong>Error: </strong>${error}</span>`;
                    loginErrAlter.style.display = "block";
                } else {
                    loginErrAlter.style.display = "none";
                }
            });
        }
    });
});

const signupForm = document.getElementById("signup-form");
const signupButton = document.getElementById("signup-form-submit");
const signupErrAlter = document.getElementById("signup-err-alert");

signupButton.addEventListener("click", (e) => {
    e.preventDefault();

    const email = signupForm.email.value;
    const password = signupForm.password.value;
    const requires2FA = signupForm.twoFA.checked;

    fetch("/signup", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password, requires2FA }),
    }).then((response) => {
        if (response.ok) {
            signupForm.email.value = "";
            signupForm.password.value = "";
            signupForm.twoFA.checked = false;
            signupErrAlter.style.display = "none";
            alert("You have successfully created a user.");
            loginSection.style.display = "block";
            twoFASection.style.display = "none";
            signupSection.style.display = "none";
        } else {
            response.json().then(({ error }) => {
                if (error !== undefined && error !== null && error !== "") {
                    signupErrAlter.innerHTML = `<span><strong>Error: </strong>${error}</span>`;
                    signupErrAlter.style.display = "block";
                } else {
                    signupErrAlter.style.display = "none";
                }
            });
        }
    });
});

const twoFAForm = document.getElementById("2fa-form");
const twoFAButton = document.getElementById("2fa-form-submit");
const twoFAErrAlter = document.getElementById("2fa-err-alert");

twoFAButton.addEventListener("click", (e) => {
    e.preventDefault();

    const email = twoFAForm.email.value;
    const loginAttemptId = twoFAForm.login_attempt_id.value;
    const twoFACode = twoFAForm.email_code.value;

    fetch("/verify-2fa", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, loginAttemptId, "2FACode": twoFACode }),
    }).then((response) => {
        if (response.ok) {
            twoFAForm.email.value = "";
            twoFAForm.email_code.value = "";
            twoFAForm.login_attempt_id.value = "";
            twoFAErrAlter.style.display = "none";
            alert("You have successfully logged in.");
            loginSection.style.display = "block";
            twoFASection.style.display = "none";
            signupSection.style.display = "none";
        } else {
            response.json().then(({ error }) => {
                if (error !== undefined && error !== null && error !== "") {
                    twoFAErrAlter.innerHTML = `<span><strong>Error: </strong>${error}</span>`;
                    twoFAErrAlter.style.display = "block";
                } else {
                    twoFAErrAlter.style.display = "none";
                }
            });
        }
    });
});
