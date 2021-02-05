interface String
{
    isNullOrEmpty(): boolean;
}

String.prototype.isNullOrEmpty = function(): boolean {
    return !this || this.length == 0;
}

const urlRegex = /((([A-Za-z]{3,9}:(?:\/\/)?)(?:[\-;:&=\+\$,\w]+@)?[A-Za-z0-9\.\-]+|(?:www\.|[\-;:&=\+\$,\w]+@)[A-Za-z0-9\.\-]+)((?:\/[\+~%\/\.\w\-_]*)?\??(?:[\-\+=&;%@\.\w_]*)#?(?:[\.\!\/\\\w]*))?(:\d{1,5})?)/;

//return color for labels/tags based on how dark background color is
function textColor(color: string): string {
    color = getHexColor(color);
	//if only first half of color is defined, repeat it
	if(color.length < 5) {
		color += color.slice(1);
	}
	return (Number(color.replace('#','0x'))) > (0xffffff/2) ? '#333' : '#fff';
}

function getHexColor(color: string): string {
    if (color.indexOf('#') != -1) return color;
    
    var reg = color.match(/^rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*(\d+))?\)$/);
    if (reg == null)
        return "#00";
    
    const hex = (x: any) => ("0" + parseInt(x).toString(16)).slice(-2);
    return '#' + hex(reg[1]) + hex(reg[2]) + hex(reg[3]);
};

// Set text color of every label or tag according to its background-color
document.addEventListener('DOMContentLoaded', function() {
    var elements = document.getElementsByClassName("tag");
    if (elements != null) {
        for (var i = 0; i < elements.length; i++) {
            var element = <HTMLElement>elements[i];
            if (element.style.backgroundColor) {
                element.style.color = textColor(element.style.backgroundColor);
            }
        }
    }
});

window.onclick = function(event: MouseEvent) {
    var target = <HTMLElement>event.target;
    var parentId = "";
    var root = getDropdownRoot(target);
    if (target != null && root != null) {
        parentId = (root.id != target.id) ? root.id : "";
        if (target.id.includes("dropdown-trigger")) {
            root.classList.toggle("is-active");
        }
    }
    deactivateClassesExceptId(parentId, ["dropdown", "has-dropdown"]);
}

/**
 * @param {Element} element Element of which you want to know if it's part of a dropdown.
 */
function isPartOfDropdown(element: HTMLElement): boolean {
    return getDropdownRoot(element) != null;
}

type RootIdentifier = (element: HTMLElement) => boolean;

function getRootElementByExpression(element: HTMLElement, expression: RootIdentifier): HTMLElement | null {
    var root: HTMLElement | null = element;
    while (root != null && !expression(root)) {
        root = root.parentElement;
    }
    return root;
}

/**
 * Returns the root element of the given dropdown child if one was found. Else returns null.
 * @param {Element} element Element which should be a child of a dropdown and from which we start to searching for the dropdown root element.
 */
function getDropdownRoot(element: HTMLElement): HTMLElement | null {
    return getRootElementByExpression(element, (ele: HTMLElement) : boolean => {
        return ele.classList.contains("dropdown") || ele.classList.contains("has-dropdown");
    });
}

/**
 * Returns the root element of the given label-box child if one was found. Else returns null.
 * @param {Element} element Element which should be a child of a label-box `li` and from which we start to searching for the dropdown root element.
 */
function getLabelElementRoot(element: HTMLElement): HTMLElement | null {
    return getRootElementByExpression(element, (ele: HTMLElement) : boolean => {
        return ele.tagName == "LI" && ele.id.startsWith("label-box-");
    });
}

/**
 * @param {string} id Element id that won't be deactivated, even if the class matches.
 * @param {string} cssClasses Css classes of the elements we want to deactivate.
 */
function deactivateClassesExceptId(id: string, cssClasses: string[]): void {
    var elements = Array.from(document.getElementsByClassName(cssClasses[0]));
    
    for (var i = 1; i < cssClasses.length; i++) {
        elements = elements.concat(Array.from(document.getElementsByClassName(cssClasses[i])));
    }
    
    for (var i = 0; i < elements.length; i++) {
        var activeElement = elements[i];
        if (activeElement.id != id) {
            activeElement.classList.remove('is-active');
        }
    }
}

/**
 * @param {Element} element Needs to be the expand button of an issue in the issues overview.
 */
function expandPreview(element: HTMLElement): void {
    if (element.classList.contains('toggle')) {
        var icon = element.querySelector('.zg-icon');
        if (icon != null) {
            icon.classList.toggle('zg-triangle-down');
            icon.classList.toggle('zg-triangle-up');
        }
        var root = getRootElementByExpression(element, (ele: HTMLElement) : boolean => {
            return ele.classList.contains('issue') || ele.classList.contains('project-header');
        });
        if (root != null) {
            var issueBody = root.querySelector('.issue-body') || root.querySelector('h4.project-header .description');
            if (issueBody != null)
                issueBody.classList.toggle("is-hidden");
        }
    }
}

// Issues Page issue content preview button
document.addEventListener('DOMContentLoaded', function() {
    var issuesBox = document.getElementById("issues-box") || document.getElementById("project-header");
    if (issuesBox != null) {
        var toggleButtons = issuesBox.getElementsByClassName("toggle");
        for (var i = 0; i < toggleButtons.length; i++) {
            (<HTMLElement>toggleButtons[i]).onclick = function() { expandPreview(<HTMLElement>this) };
        }
    }
});

// New Issue Page write and preview markdown tabs
document.addEventListener('DOMContentLoaded', function() {
    var newForm = document.getElementById("new-issue-form") || document.getElementById("new-comment-form");
    if (newForm != null) {
        var newIssueTabs = newForm.querySelectorAll("[id=tab-write], [id=tab-preview]");
        newIssueTabs.forEach(function(navEl) {
            (<HTMLElement>navEl).onclick = function() { toggleTab(<HTMLElement>this); }
        });

        
    }
});

function toggleTab(element: HTMLElement): void {
    var newForm = document.getElementById("new-issue-form") || document.getElementById("new-comment-form");
    
    if (newForm != null) {
        newForm.querySelectorAll("[id=tab-write], [id=tab-preview]")
            .forEach(function(navEl) {
                if (navEl.id == element.id) {
                    navEl.classList.add("is-active");
                } else {
                    navEl.classList.remove("is-active");
                }
            });

        newForm.querySelectorAll("[id^=tab-content-]")
            .forEach(function(content) {
                var tabContent = <HTMLElement> content;
                if (tabContent.id == element.dataset.tabContent) {
                    if (tabContent.id.includes("preview") && tabContent.classList.contains("is-hidden")) {
                        var textArea = tabContent.previousElementSibling && <HTMLTextAreaElement>tabContent.previousElementSibling.firstElementChild;
                        if (textArea != null && textArea.value) {
                            if (textArea.value.isNullOrEmpty()) {
                                tabContent.innerText = "Nothing to preview"
                            } else {
                                let url = window.location.href.replace(/\/issues\/(new|\d+)/, "/markdown");
                                let request = {
                                    headers: {
                                        "content-type": "application/x-www-form-urlencoded",
                                    },
                                    method: "POST",
                                    body: textArea.value,
                                };
                                
                                tabContent.innerHTML = "Loading...";
                                fetch(url, request)
                                    .then(response => response.text())
                                    .then(markdown => tabContent.innerHTML = markdown);
                            }
                        }
                    }
                    tabContent.classList.remove("is-hidden");
                } else {
                    tabContent.classList.add("is-hidden");
                }
            });
    }
}

// New Issue Page select, deselect and remove labels
document.addEventListener('DOMContentLoaded', function() {
    var issueForm = <HTMLFormElement>(document.getElementById("new-issue-form") || document.getElementById("update-issue-form"));
    if (issueForm != null) {
        var issueLabelsWrapper = document.getElementById("issue-labels-wrapper");
        if (issueLabelsWrapper != null) {
            var dropdownLabels = Array.from(issueLabelsWrapper.querySelectorAll("a.panel-block")).filter(a => a.id != "labels-edit");
            dropdownLabels.forEach(function(navEl) {
                (<HTMLElement>navEl).onclick = function() {
                    toggleTags(issueForm, <HTMLElement>this, issueForm.label_ids);
                }
            });
            var labels = issueLabelsWrapper.querySelectorAll("div.labels-box .tag .delete");
            labels.forEach(function(navEl) {
                (<HTMLElement>navEl).onclick = function() {
                    var parent = (<HTMLElement>this).parentElement;
                    if (parent != null)
                        toggleTags(issueForm, parent, issueForm.label_ids);
                }
            });
        }
    }
});

// New Issue Page select, deselect and remove assignees
document.addEventListener('DOMContentLoaded', function() {
    var issueForm = <HTMLFormElement>(document.getElementById("new-issue-form") || document.getElementById("update-issue-form"));
    if (issueForm != null) {
        var issueAssigneesWrapper = document.getElementById("issue-assignees-wrapper");
        if (issueAssigneesWrapper != null) {
            var dropdownAssignees = Array.from(issueAssigneesWrapper.querySelectorAll("a.panel-block"));
            dropdownAssignees.forEach(function(navEl) {
                (<HTMLElement>navEl).onclick = function() {
                    toggleTags(issueForm, <HTMLElement>this, issueForm.assignee_ids);
                }
            });
            var assignees = issueAssigneesWrapper.querySelectorAll("div.labels-box .tag .delete");
            assignees.forEach(function(navEl) {
                (<HTMLElement>navEl).onclick = function() {
                    var parent = (<HTMLElement>this).parentElement;
                    if (parent != null)
                        toggleTags(issueForm, parent, issueForm.assignee_ids);
                }
            });
        }
    }
});

// Labels Page label edit button
document.addEventListener('DOMContentLoaded', function() {
    initNewLabelBox();

    // init all buttons and actions of all labels in the labels-box
    var labelsBox = document.getElementById("labels-box");
    if (labelsBox != null) {
        var labels = labelsBox.querySelectorAll("[id^=label-box-]");

        for (var i = 0; i < labels.length; i++) {
            var labelButtonsBox = labels[i].getElementsByClassName("label-header-right")[0];
            // link edit button with function
            if (labelButtonsBox.firstElementChild == null)
                continue;
            (<HTMLElement>labelButtonsBox.firstElementChild).onclick = function() {
                var root = getLabelElementRoot(<HTMLElement>this);
                if (root == null)
                    return;
                root.getElementsByClassName("description")[0].classList.add("is-hidden");
                root.getElementsByClassName("label-body")[0].classList.remove("is-hidden");
            };
            // link delete button with function
            if (labelButtonsBox.children[1] == null)
                continue;
            (<HTMLElement>labelButtonsBox.children[1]).onclick = function() {
                var root = getLabelElementRoot(<HTMLElement>this);
                if (root == null)
                    return;
                var deleteForm = <HTMLFormElement>root.getElementsByClassName("label-body")[0].firstElementChild;
                if (deleteForm == null)
                    return;
                var action = deleteForm.action.split("/");
                action.splice(action.length - 1);
                deleteForm.action = action.join("/");
                var input = document.createElement("input");
                input.type = "hidden";
                input.name = "_method";
                input.value = "DELETE"
                deleteForm.insertBefore(input, deleteForm.firstElementChild);
                deleteForm.submit();
            };
            // link save button with function
            var saveButton = <HTMLButtonElement>labels[i].querySelector("[name=save]");
            saveButton.onclick = function() {
                var root = getLabelElementRoot(<HTMLElement>this);
                if (root == null)
                    return;
                var updateForm = <HTMLFormElement>root.getElementsByClassName("label-body")[0].firstElementChild;
                if (updateForm == null)
                    return;
                var input = document.createElement("input");
                input.type = "hidden";
                input.name = "_method";
                input.value = "PUT"
                updateForm.insertBefore(input, updateForm.firstElementChild);
                updateForm.submit();
            };

            var colorInput = <HTMLInputElement>labelsBox.querySelector("input[name=color]");
            colorInput.setAttribute("data-previous", colorInput.value)
            colorInput.oninput = function() {
                var colInput = <HTMLInputElement>this;
                if (colInput.validity.valid && colInput.previousElementSibling && colInput.previousElementSibling.firstElementChild) {
                    (<HTMLElement>colInput.previousElementSibling).style.backgroundColor = colInput.value;
                    (<HTMLInputElement>colInput.previousElementSibling.firstElementChild).value = colInput.value;

                    var root = getLabelElementRoot(colInput);
                    if (root == null)
                        return;
                    var labelTag = <HTMLElement>root.getElementsByClassName("tag")[0];
                    labelTag.style.backgroundColor = colInput.value;
                    labelTag.style.color = textColor(colInput.value);
                }
            };

            var colorBoxInput = <HTMLInputElement>labelsBox.querySelector("input[type=color]");
            colorBoxInput.setAttribute("data-previous", colorBoxInput.value)
            colorBoxInput.oninput = function() {
                var colInput = <HTMLInputElement>this;
                var parent = <HTMLElement>colInput.parentElement;
                if (parent && parent.nextElementSibling) {
                    parent.style.backgroundColor = colInput.value;
                    (<HTMLInputElement>parent.nextElementSibling).value = colInput.value;

                    var root = getLabelElementRoot(colInput);
                    if (root == null)
                        return;
                    var labelTag = <HTMLElement>root.getElementsByClassName("tag")[0];
                    labelTag.style.backgroundColor = colInput.value;
                    labelTag.style.color = textColor(colInput.value);
                }
            };

            var nameInput = <HTMLInputElement>labelsBox.querySelector("input[name=name]");
            nameInput.setAttribute("data-previous", nameInput.value)
            nameInput.oninput = function() {
                var input = <HTMLInputElement>this;
                var root = getLabelElementRoot(input);
                if (root == null)
                    return;
                var labelTag = <HTMLElement>root.getElementsByClassName("tag")[0];
                labelTag.innerText = input.value;
            };

            var descriptionInput = <HTMLInputElement>labelsBox.querySelector("input[name=description]");
            descriptionInput.setAttribute("data-previous", descriptionInput.value)
            descriptionInput.oninput = function() {
                var input = <HTMLInputElement>this;
                var root = getLabelElementRoot(input);
                if (root == null)
                    return;
                var description = <HTMLElement>root.getElementsByClassName("description")[0];
                description.innerText = input.value;
            };
            
            // link cancel button with function
            var cancelButton = <HTMLButtonElement>labels[i].querySelector("[name=cancel]");
            cancelButton.onclick = function() {
                if (descriptionInput.hasAttribute("data-previous"))
                    descriptionInput.value = descriptionInput.getAttribute("data-previous") as string;
                if (nameInput.hasAttribute("data-previous"))
                    nameInput.value = nameInput.getAttribute("data-previous") as string;
                if (colorInput.hasAttribute("data-previous"))
                    colorInput.value = colorInput.getAttribute("data-previous") as string;
                if (colorBoxInput.hasAttribute("data-previous")) {
                    colorBoxInput.value = colorBoxInput.getAttribute("data-previous") as string;
                    if (colorBoxInput.parentElement != null)
                        colorBoxInput.parentElement.style.backgroundColor = colorBoxInput.value;
                }
                var root = getLabelElementRoot(<HTMLElement>this);
                if (root == null)
                    return;
                var labelTag = <HTMLElement>root.getElementsByClassName("tag")[0];
                labelTag.innerText = nameInput.value;
                labelTag.style.backgroundColor = colorInput.value;
                labelTag.style.color = textColor(colorInput.value);
                var description = <HTMLElement>root.getElementsByClassName("description")[0];
                description.innerText = descriptionInput.value;
                root.getElementsByClassName("description")[0].classList.remove("is-hidden");
                root.getElementsByClassName("label-body")[0].classList.add("is-hidden");
            };
        }
    }
});

function initNewLabelBox() {
    var newLabelDiv = <HTMLElement>document.getElementById("new-label");
    if (newLabelDiv != null) {
        var newLabelBtn = <HTMLElement>document.getElementById("new-label-button")
        newLabelBtn.onclick = function () {
            newLabelDiv.classList.remove("is-hidden")
        };
        
        var newLabelColor = <HTMLElement>document.getElementById("new-label-color");
        var newLabelColorInp = newLabelColor.querySelector<HTMLInputElement>("input");
        var newLabelTag = <HTMLElement>document.getElementById("new-label-tag");
        var colorInput = <HTMLInputElement>document.getElementById("new-label-color-input");
        colorInput.oninput = function() {
            var colInput = <HTMLInputElement>this;
            if (colInput.validity.valid && newLabelColorInp) {
                newLabelColor.style.backgroundColor = colInput.value;
                newLabelColorInp.value = colInput.value;
                newLabelTag.style.backgroundColor = colInput.value;
                newLabelTag.style.color = textColor(colInput.value);
            }
        };

        if (newLabelColorInp != null) {
            newLabelColorInp.oninput = function() {
                var colInput = <HTMLInputElement>this;
                var parent = <HTMLElement>colInput.parentElement;
                if (parent) {
                    newLabelColor.style.backgroundColor = colInput.value;
                    colorInput.value = colInput.value;
                    newLabelTag.style.backgroundColor = colInput.value;
                    newLabelTag.style.color = textColor(colInput.value);
                }
            };
        }
        
        var newLabelInput = <HTMLInputElement>document.getElementById("new-label-name-input");
        newLabelInput.oninput = function() {
            newLabelTag.innerText = (<HTMLInputElement>this).value;
        };
        var cancelButton = <HTMLElement>newLabelDiv.querySelector("[name=cancel]");
        cancelButton.onclick = function() {
            newLabelDiv.classList.add("is-hidden");
            newLabelTag.innerText = "Label preview";
            colorInput.value = "#00d1b2";
            if (newLabelColorInp)
                newLabelColorInp.value = "#00d1b2";
            newLabelColor.style.backgroundColor = "#00d1b2";
            newLabelTag.style.backgroundColor = "#00d1b2";
        };
    }
}

// New Project Page choose owner
document.addEventListener('DOMContentLoaded', function() {
    var newProjectForm = <HTMLFormElement>document.getElementById("new-project-form");
    if (newProjectForm != null) {
        var ownerDropdownMenu = <HTMLElement>document.getElementById("owner-dropdown-menu");
        var dropdownOwners = Array.from(ownerDropdownMenu.querySelectorAll("a.panel-block"));
        dropdownOwners.forEach(function(navEl) {
            (<HTMLElement>navEl).onclick = function() {
                var name = (<HTMLElement>this).getAttribute("name");
                if (name != null)
                    toggleNewProjectDropdowns(newProjectForm, name, "[name^=o_]", "[id=owner-dropdown-trigger]", newProjectForm.owner_id);
            }
        });

        var vcsDropdownMenu = <HTMLElement>document.getElementById("vcs-dropdown-menu");
        var dropdownVcs = Array.from(vcsDropdownMenu.querySelectorAll("a.panel-block"));
        dropdownVcs.forEach(function(navEl) {
            (<HTMLElement>navEl).onclick = function() {
                var name = (<HTMLElement>this).getAttribute("name");
                if (name != null)
                    toggleNewProjectDropdowns(newProjectForm, name, "[name^=v_]", "[id=vcs-dropdown-trigger]", newProjectForm.vcs);
            }
        });
    }
});

function toggleNewProjectDropdowns(newProjectForm: HTMLFormElement, id: string, entrySelector: string, triggerSelector: string, valueTarget: any) {
    var dropdownEntries = newProjectForm.querySelectorAll(entrySelector);
    var dropdownEntry;
    for (var i = 0; i < dropdownEntries.length; i++) {
        var entry = <HTMLElement>dropdownEntries[i];
        if (entry.getAttribute("name") == id) {
            entry.classList.add("is-active");
            var icons = entry.getElementsByClassName("panel-icon");
            for (var j = 0; j < icons.length; j+=2) {
                icons[j].classList.remove("is-invisible");
            }
            dropdownEntry = entry;
        } else {
            entry.classList.remove("is-active");
            var icons = entry.getElementsByClassName("panel-icon");
            for (var j = 0; j < icons.length; j+=2) {
                icons[j].classList.add("is-invisible");
            }
        }
    }
    valueTarget.value = id.substring(2, id.length);
    var dropdownTrigger = <HTMLElement>newProjectForm.querySelector(triggerSelector);
    if (dropdownTrigger.firstElementChild && dropdownEntry && dropdownEntry.firstElementChild) {
        if (dropdownEntry.firstElementChild.nextElementSibling && dropdownTrigger.firstElementChild.nextSibling && dropdownEntry.firstElementChild.nextElementSibling.nextSibling){
            var oldImg = dropdownTrigger.querySelector<HTMLImageElement>("img");
            var newImg = dropdownEntry.querySelector<HTMLImageElement>("img");
            if (oldImg != null && newImg != null)
                oldImg.src = newImg.src;
            var oldText = dropdownTrigger.lastElementChild as HTMLDataElement;
            var newText = dropdownEntry.lastElementChild as HTMLDataElement;
            if (oldText != null && newText != null)
                oldText = newText;
        }
    }
}

// Clone Project input
document.addEventListener('DOMContentLoaded', function() {
    var cloneUrlInput = <HTMLInputElement>document.getElementById("project-clone-url");
    if (cloneUrlInput != null) {
        var cloneUrlClipboardBtn = <HTMLElement>document.getElementById("project-clone-clipboard");
        cloneUrlClipboardBtn.onclick = function() {
            copyToClipboard(cloneUrlInput.value);
        };
        var urlElements = Array.from(document.getElementsByClassName("clone-url"));
        urlElements.push(cloneUrlInput);
        var httpsBtn = <HTMLElement>document.getElementById('project-clone-https');
        var sshBtn = <HTMLElement>document.getElementById('project-clone-ssh');
        [httpsBtn, sshBtn].forEach(function(btn) {
            btn.onclick = function() {
                var data = (<HTMLElement>this).getAttribute("data-url");
                if (!btn.classList.contains('is-active') && data) {
                    toggleCloneUrlBtn(httpsBtn);
                    toggleCloneUrlBtn(sshBtn);
                    setText(urlElements, data);
                }
            };
        });
    }
});

function toggleCloneUrlBtn(btn: HTMLElement) {
    btn.classList.toggle('is-primary');
    btn.classList.toggle('is-outlined');
    btn.classList.toggle('is-active');
}

// Issue Page edit labels and assignees
document.addEventListener('DOMContentLoaded', function() {
    var updateIssueForm = <HTMLFormElement>document.getElementById("update-issue-form");
    if (updateIssueForm != null) {
        var input = document.createElement("input");
        input.type = "hidden";
        input.name = "_method";
        input.value = "PUT"
        updateIssueForm.insertBefore(input, updateIssueForm.firstElementChild);

        var labelsDropdown = updateIssueForm.querySelector("[id=labels-dropdown]");
        var assigneesDropdown = updateIssueForm.querySelector("[id=assignees-dropdown]");

        if (labelsDropdown != null && assigneesDropdown != null) {
            // Callback function to execute when mutations are observed
            var mutationCallbackLabels = (mutationsList: any) => {
                for(let mutation of mutationsList) {
                    if (mutation.type === 'attributes' && mutation.attributeName === 'class') {
                        if(!mutation.target.classList.contains('is-active')) {
                            updateIssueForm.submit();
                        }
                    }
                }
            };
            var mutationCallbackAssignees = Object.assign([], mutationCallbackLabels);

            // Create an observer instance linked to the callback function
            var labelsObserver = new MutationObserver(mutationCallbackLabels);
            var assigneesObserver = new MutationObserver(mutationCallbackAssignees);
            labelsObserver.observe(labelsDropdown, { attributes: true });
            assigneesObserver.observe(assigneesDropdown, { attributes: true });
        }
    }
});

// User Settings Account Page change email notification
document.addEventListener('DOMContentLoaded', function() {
    var notificationForms = document.querySelectorAll<HTMLFormElement>("[id^=email-notification-]");
    if (notificationForms != null) {
        notificationForms.forEach(notificationForm => {
            var selectElement = notificationForm.querySelector<HTMLSelectElement>("select");
            if (selectElement != null) {
                selectElement.onchange = () => { notificationForm.submit(); };
            }
        });
    }
});

// Project Settings Page change visibility
document.addEventListener('DOMContentLoaded', function() {
    var basicSettingsForm = <HTMLFormElement>document.getElementById("project_basic_settings");
    if (basicSettingsForm != null) {
        var visibilityDropdownMenu = <HTMLElement>document.getElementById("visibility-dropdown-menu");
        var dropdownVisibility = Array.from(visibilityDropdownMenu.querySelectorAll("a.panel-block"));
        dropdownVisibility.forEach(function(navEl) {
            (<HTMLElement>navEl).onclick = function() {
                var name = (<HTMLElement>this).getAttribute("name");
                if (name != null)
                    selectDropdownEntry(basicSettingsForm, name, "[name^=v_]", "[id=visibility-dropdown-trigger]", basicSettingsForm.is_private);
            }
        });
        
    }
});

// Project Settings and Project Page topics management
document.addEventListener('DOMContentLoaded', function() {
    let searchRequest = {
        headers: {
            "Accept": "text/html; text/plain",
        },
        method: "GET",
    };
    let addRequest = {
        headers: {
            "content-type": "application/x-www-form-urlencoded",
        },
        method: "POST",
        body: "",
    };
    var delayTimer: number;
    var basicSettingsForm = <HTMLFormElement>document.getElementById("project_basic_settings");
    var manageTopicsForm = <HTMLFormElement>document.getElementById("project-manage-topics-form");
    var topicsBox = document.getElementById("topics-tags-box");
    if (manageTopicsForm != null && topicsBox != null) {
        //hookup manag topics anchor
        var manageElement = <HTMLAnchorElement>document.getElementById("project-manage-topics");
        var manageButton = <HTMLAnchorElement>document.getElementById("topics-manage");
        if (manageButton != null) {
            manageButton.onclick = function() {
                if (manageElement != null && topicsBox != null) {
                    topicsBox.querySelectorAll(".tag .delete").forEach(tag => tag.classList.remove("is-hidden"));
                    manageButton.classList.add("is-hidden");
                    manageElement.classList.remove("is-hidden")
                }
            }
        }
        var submitButton = <HTMLAnchorElement>manageTopicsForm.querySelector("[id=topic-submit]");
        if (submitButton != null) {
            submitButton.onclick = function() {
                if (manageElement != null) {
                    let url = manageTopicsForm.action;
                    var request = {
                        headers: { "content-type": "application/x-www-form-urlencoded" },
                        method: "PUT",
                        body: "topic_ids=" + manageTopicsForm.topic_ids.value,
                    };
                    fetch(url, request)
                        .then(response => {
                            if (response.status == 200) {
                                if (topicsBox != null)
                                    topicsBox.querySelectorAll(".tag .delete").forEach(tag => tag.classList.add("is-hidden"));
                                manageButton.classList.remove("is-hidden");
                                manageElement.classList.add("is-hidden")
                            }
                        });
                }
            }
        }
    }

    var topicsForm = manageTopicsForm ? manageTopicsForm : basicSettingsForm;
    if (topicsForm != null && topicsBox != null) {
        var topicsField = document.getElementById("topics-field");
        if (topicsField != null) {
            var topics = topicsBox.querySelectorAll(".tag .delete");
            topics.forEach(function(navEl) {
                (<HTMLElement>navEl).onclick = function() {
                    var parent = (<HTMLElement>this).parentElement;
                    if (parent != null) {
                        toggleTags(topicsForm, parent, topicsForm.topic_ids);
                        parent.remove();
                    }
                }
            });

            var topicsSearch = <HTMLInputElement>document.getElementById("topics-search");
            var topicsSearchAutocomplete = <HTMLElement>document.getElementById("topics-search-autocomplete");
            var topicsSearchAutocompleteContent = <HTMLElement>topicsSearchAutocomplete.querySelector(".dropdown-content");
            if (topicsSearch != null && topicsSearchAutocomplete != null && topicsSearchAutocompleteContent != null) {
                let url = window.location.origin + "/topics";
                 
                topicsSearch.addEventListener("keydown", function(event) {
                    if (event.key === "Enter") {
                        event.preventDefault();
                        addRequest.body = "topic_names=" + topicsSearch.value;//encodeURIComponent(topicsSearch.value);

                        interface Topic {
                            id: number;
                            name: string;
                        }

                        fetch(url + "/new", addRequest)
                            .then(response => response.text())
                            .then(tops => { 
                                var topics: Topic[] = JSON.parse(tops);
                                for (var topic of topics) {
                                    var tagA = document.createElement('span') as HTMLSpanElement;
                                    tagA.classList.add("delete");
                                    tagA.classList.add("is-small");
                                    tagA.onclick = function() {
                                        var parent = (<HTMLElement>this).parentElement;
                                        if (parent != null) {
                                            toggleTags(topicsForm, parent, topicsForm.topic_ids);
                                            parent.remove();
                                        }
                                    }
                                    
                                    var tagSpan = document.createElement('span') as HTMLSpanElement;
                                    tagSpan.setAttribute("name", `t_${topic.id}`);
                                    tagSpan.setAttribute("data-value", `${topic.id}`);
                                    tagSpan.classList.add("tag");
                                    tagSpan.innerText = topic.name;
                                    tagSpan.appendChild(tagA);
                                    
                                    if (topicsForm.topic_ids.value.trim().isNullOrEmpty()) {
                                        topicsForm.topic_ids.value = topic.id;
                                    }
                                    else {
                                        let values = [topicsForm.topic_ids.value];
                                        values.push(topic.id);
                                        topicsForm.topic_ids.value = values.join();
                                    }
        
                                    topicsBox?.appendChild(tagSpan);
                                }
                                topicsSearch.value = "";
                                topicsSearchAutocomplete.classList.remove("is-block");
                            });

                    }
                });

                topicsSearch.oninput = function () {
                    clearTimeout(delayTimer);
                    topicsSearch.parentElement?.classList.add("is-loading");
                    delayTimer = setTimeout(function() {
                        fetch(url + "/search?q=" + topicsSearch.value, searchRequest)
                            .then(response => response.text())
                            .then(searchResults => {
                                topicsSearchAutocompleteContent.innerHTML = searchResults.isNullOrEmpty() ? "<span class=\"dropdown-item\">No matching topics found.\nPress enter to add your input as new topic.</span>" : searchResults;
                                var autoItems = topicsSearchAutocompleteContent.querySelectorAll<HTMLAnchorElement>("a[data-value].dropdown-item");

                                autoItems.forEach(item => {
                                    item.onclick = function () {
                                        var tagA = document.createElement('a') as HTMLAnchorElement;
                                        tagA.classList.add("delete");
                                        tagA.classList.add("is-small");
                                        tagA.onclick = function() {
                                            var parent = (<HTMLElement>this).parentElement;
                                            if (parent != null) {
                                                toggleTags(topicsForm, parent, topicsForm.topic_ids);
                                                parent.remove();
                                            }
                                        }
                                        
                                        var tagSpan = document.createElement('span') as HTMLSpanElement;
                                        tagSpan.setAttribute("name", item.getAttribute("name") as string);
                                        tagSpan.setAttribute("data-value", item.getAttribute("data-value") as string);
                                        tagSpan.classList.add("tag");
                                        tagSpan.innerText = item.innerText;
                                        tagSpan.appendChild(tagA);
                                        
                                        if (topicsForm.topic_ids.value.trim().isNullOrEmpty()) {
                                            topicsForm.topic_ids.value = item.getAttribute("data-value") as string;
                                        }
                                        else {
                                            let values = [topicsForm.topic_ids.value];
                                            values.push(item.getAttribute("data-value") as string);
                                            topicsForm.topic_ids.value = values.join();
                                        }
            
                                        topicsBox?.appendChild(tagSpan);
                                        topicsSearch.value = "";
                                        topicsSearchAutocomplete.classList.remove("is-block");
                                    }
                                })

                                topicsSearch.parentElement?.classList.toggle("is-loading");
                            });
                        
                        topicsSearchAutocomplete.classList.add("is-block");
                    }, 500);
                }
            }
        }
    }
});

// Project Settings Page Setup Modal Dialogs
document.addEventListener('DOMContentLoaded', function() {
    var dangerZone = <HTMLElement>document.getElementById("danger-zone");
    if (dangerZone != null) {
        var buttons = Array.from(dangerZone.querySelectorAll<HTMLButtonElement>("button[id^=modal-show-]")).filter(button => button.hasAttribute("data-target"));
        buttons.forEach(button => {
            button.onclick = function () {
                var target = button.getAttribute("data-target");
                if (target != null)
                    document.getElementById(target)?.classList.toggle("is-active");

                var modal = getRootElementByExpression(button, (ele: HTMLElement) : boolean => {
                        return ele.classList.contains("modal");
                    });
                if (modal != null) {
                    var submitButton = <HTMLButtonElement>modal.querySelector("button[id^=modal-submit-]");
                    submitButton.disabled = true;
                }
            }
        });

        var buttons = Array.from(dangerZone.querySelectorAll<HTMLButtonElement>("button[id^=modal-close-]")).filter(button => button.hasAttribute("data-target"));
        buttons.forEach(button => {
            button.onclick = function () {
                var target = button.getAttribute("data-target");
                if (target != null)
                    document.getElementById(target)?.classList.toggle("is-active");
            
                var modal = getRootElementByExpression(button, (ele: HTMLElement) : boolean => {
                        return ele.classList.contains("modal");
                    });
                if (modal != null) {
                    var submitButton = <HTMLButtonElement>modal.querySelector("button[id^=modal-submit-]");
                    submitButton.disabled = true;
                }
            }
        });
    }
});

// Project Settings Page Transfer Ownership
document.addEventListener('DOMContentLoaded', function() {
    var transferForm = <HTMLFormElement>document.getElementById("project-transfer");
    var submitButton = <HTMLButtonElement>document.getElementById("modal-submit-transfer");
    if (transferForm != null && submitButton != null) {
        (transferForm.confirm_name as HTMLInputElement).oninput = function () {
                if (transferForm.confirm_name.value == transferForm.project_name.value) {
                    submitButton.disabled = false;
                }
                else {
                    submitButton.disabled = true;
                }
            };
    }
});

// Project Settings Page Delete Project
document.addEventListener('DOMContentLoaded', function() {
    var deleteForm = <HTMLFormElement>document.getElementById("project-delete");
    var submitButton = <HTMLButtonElement>document.getElementById("modal-submit-delete");
    if (deleteForm != null && submitButton != null) {
        (deleteForm.confirm_name as HTMLInputElement).oninput =  function () {
                if (deleteForm.confirm_name.value == deleteForm.project_name.value) {
                    submitButton.disabled = false;
                }
                else {
                    submitButton.disabled = true;
                }
            };
    }
});

/**
 * Sets the clicked entry of a dropdown to active and makes its selection icon visible, if there is one. It also sets the data-value of that
 * entry to the given valueTargets value.
 * @param {HTMLFormElement} parentForm The form of with the dropdown is a part of.
 * @param {string} clickedEntryId The id of the clicked dropdown entry. This means you have to subscribe to the dropdowns onClick event manually.
 * @param {string} entrySelector The CSS selector string for the dropdowns entries.
 * @param {string} triggerSelector The CSS selector for the dropdown trigger.
 * @param {any} valueTarget The element on which the value of the clicked entry's data-value will be set. Should the dropdowns corresponding HTMLInputElement.
 */
function selectDropdownEntry(parentForm: HTMLFormElement, clickedEntryId: string, entrySelector: string, triggerSelector: string, valueTarget: any) {
    var dropdownEntries = parentForm.querySelectorAll(entrySelector);
    var dropdownEntry;
    // Visually select correct dropdown entry
    for (var i = 0; i < dropdownEntries.length; i++) {
        var entry = <HTMLElement>dropdownEntries[i];
        if (entry.getAttribute("name") == clickedEntryId) {
            entry.classList.add("is-active");
            var icons = entry.getElementsByClassName("panel-icon");
            for (var j = 0; j < 1; j++) {
                icons[j].classList.remove("is-invisible");
            }
            dropdownEntry = entry;
        } else {
            entry.classList.remove("is-active");
            var icons = entry.getElementsByClassName("panel-icon");
            for (var j = 0; j < 1; j++) {
                icons[j].classList.add("is-invisible");
            }
        }
    }

    if (dropdownEntry) {
        // Set value of target input
        valueTarget.value = dropdownEntry.getAttribute("data-value");
        // Change dropdown-trigger to represent selected value
        var dropdownTrigger = <HTMLElement>parentForm.querySelector(triggerSelector);
        if (dropdownTrigger.firstElementChild && dropdownEntry.firstElementChild) {
            if (dropdownEntry.firstElementChild.nextElementSibling && dropdownTrigger.firstElementChild.nextSibling && dropdownEntry.firstElementChild.nextElementSibling.nextSibling){
                var oldImg = dropdownTrigger.querySelector<HTMLImageElement>("img");
                var newImg = dropdownEntry.querySelector<HTMLImageElement>("img");
                if (oldImg != null && newImg != null)
                    oldImg.src = newImg.src;

                var oldIconElement = dropdownTrigger.querySelector(".zg-icon");
                var newIconElement = dropdownEntry.querySelectorAll(".zg-icon")[1];
                if (oldIconElement != null && newIconElement != null){
                    var oldIcon = oldIconElement.classList[1];
                    var newIcon = newIconElement.classList[1];
                    oldIconElement.classList.toggle(oldIcon);
                    oldIconElement.classList.toggle(newIcon);
                }

                var oldText = dropdownTrigger.lastElementChild?.previousElementSibling;
                var newText = dropdownEntry.lastElementChild;
                if (oldText != null && newText != null)
                    setText([oldText], getText(newText));
            }
        }
    }
}

/**
 * Sets the clicked entry of a dropdown to active and makes its selection icon visible, if there is one. It also sets the data-value of that
 * entry to the given valueTargets value.
 * @param {HTMLFormElement} parentForm The form of with the tags dropdown and box is a part of.
 * @param {HTMLElement} tagElement The id of the clicked dropdown entry. This means you have to subscribe to the dropdowns onClick event manually.
 * @param {any} valueTarget The element on which the value of the clicked tag's data-value will be set. Should the dropdowns corresponding HTMLInputElement.
 */
function toggleTags(parentForm: HTMLFormElement, tagElement: HTMLElement, valueTarget: any) {
    var tags = parentForm.querySelectorAll("[name=" + tagElement.getAttribute("name") + "]");
    for (var i = 0; i < tags.length; i++) {
        var tag = <HTMLElement>tags[i];
        if (tag.classList.contains("panel-block")) {
            tag.classList.toggle("is-active");
            var icons = tag.getElementsByClassName("panel-icon");
            for (var j = 0; j < 1; j++) {
                icons[j].classList.toggle("is-invisible");
            }
        }
        else if (tag.classList.contains("tag") && tagElement.hasAttribute("data-value")) {
            var data = tagElement.getAttribute("data-value");

            if (tag.classList.contains("is-hidden")) {
                tag.classList.remove("is-hidden");
                
                if (valueTarget.value.trim().isNullOrEmpty()) {
                    valueTarget.value = data;
                }
                else {
                    let values = [valueTarget.value];
                    values.push(data);
                    valueTarget.value = values.join();
                }
            }
            else {
                tag.classList.add("is-hidden");

                let values = valueTarget.value.split(',');
                values.splice(values.indexOf(data), 1);
                valueTarget.value = values.join();
                if (parentForm.id == "update-issue-form")
                    parentForm.submit();
            }
        }
    }
}


function setText(elements: Element[], newText: string) {
    elements.forEach(function(element) {
        switch (element.nodeName) {
            case "INPUT":
                (<HTMLInputElement>element).value = newText;
                break;
            case "SPAN":
                (<HTMLSpanElement>element).innerText = newText;
                break;        
            default:
                break;
        }
    });
}

function getText(element: Element) {
    switch (element.nodeName) {
        case "INPUT":
            return (<HTMLInputElement>element).value;
        case "SPAN":
            return (<HTMLSpanElement>element).innerText;      
        default:
            return "";
    }
}

function copyToClipboard(text: string) {
    if (document.queryCommandSupported && document.queryCommandSupported("copy")) {
        var textarea = document.createElement("textarea");
        textarea.textContent = text;
        textarea.style.position = "fixed";  // Prevent scrolling to bottom of page in MS Edge.
        document.body.appendChild(textarea);
        textarea.select();
        try {
            return document.execCommand("copy");  // Security exception may be thrown by some browsers.
        } catch (ex) {
            console.warn("Copy to clipboard failed.", ex);
            return false;
        } finally {
            document.body.removeChild(textarea);
        }
    }
}