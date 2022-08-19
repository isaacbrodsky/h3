/*
 * Copyright 2022 Uber Technologies, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *         http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
/** @file faultInjection.h
 * @brief   Functions for injecting faults for testing hard to reach code paths
 */

#ifndef FAULT_INJECTION_H
#define FAULT_INJECTION_H

#include <stdbool.h>

#if ENABLE_FAULT_INJECTION

void faultInjectControl(int steps) { _faultInjectSteps = steps; }
int faultInjectState() { return _faultInjectSteps; }
bool faultInject(bool x);

#define FAULT_INJECT(x) faultInject(x)

#else  // ENABLE_FAULT_INJECTION

#define FAULT_INJECT(x) x

#endif  // ENABLE_FAULT_INJECTION

#endif  // FAULT_INJECTION_H
