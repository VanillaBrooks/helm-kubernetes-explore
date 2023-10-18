# Simple 

A single webserver serving a static HTML page on 8080

## Building

```
podman build . -t simple-webservice:0.1.0
```

## Deploying

```
helm install simple-chart/ --generate-name
```

```
helm list
```

## Chart Diff

```diff
diff --git a/simple-chart-diff/Chart.yaml b/simple/simple-chart/Chart.yaml
index 871357d..11bb7d8 100644
--- a/simple-chart-diff/Chart.yaml
+++ b/simple/simple-chart/Chart.yaml
@@ -1,5 +1,5 @@
 apiVersion: v2
-name: simple-chart-diff
+name: simple-chart
 description: A Helm chart for Kubernetes
 
 # A chart can be either an 'application' or a 'library' chart.
@@ -15,10 +15,10 @@ type: application
 # This is the chart version. This version number should be incremented each time you make changes
 # to the chart and its templates, including the app version.
 # Versions are expected to follow Semantic Versioning (https://semver.org/)
-version: 0.1.0
+version: "0.1.1"
 
 # This is the version number of the application being deployed. This version number should be
 # incremented each time you make changes to the application. Versions are not expected to
 # follow Semantic Versioning. They should reflect the version the application is using.
 # It is recommended to use it with quotes.
-appVersion: "1.16.0"
+appVersion: "0.2.0"
diff --git a/simple-chart-diff/templates/NOTES.txt b/simple/simple-chart/templates/NOTES.txt
index aa7905d..3e86deb 100644
--- a/simple-chart-diff/templates/NOTES.txt
+++ b/simple/simple-chart/templates/NOTES.txt
@@ -6,16 +6,16 @@
   {{- end }}
 {{- end }}
 {{- else if contains "NodePort" .Values.service.type }}
-  export NODE_PORT=$(kubectl get --namespace {{ .Release.Namespace }} -o jsonpath="{.spec.ports[0].nodePort}" services {{ include "simple-chart-diff.fullname" . }})
+  export NODE_PORT=$(kubectl get --namespace {{ .Release.Namespace }} -o jsonpath="{.spec.ports[0].nodePort}" services {{ include "simple-chart.fullname" . }})
   export NODE_IP=$(kubectl get nodes --namespace {{ .Release.Namespace }} -o jsonpath="{.items[0].status.addresses[0].address}")
   echo http://$NODE_IP:$NODE_PORT
 {{- else if contains "LoadBalancer" .Values.service.type }}
      NOTE: It may take a few minutes for the LoadBalancer IP to be available.
-           You can watch the status of by running 'kubectl get --namespace {{ .Release.Namespace }} svc -w {{ include "simple-chart-diff.fullname" . }}'
-  export SERVICE_IP=$(kubectl get svc --namespace {{ .Release.Namespace }} {{ include "simple-chart-diff.fullname" . }} --template "{{"{{ range (index .status.loadBalancer.ingress 0) }}{{.}}{{ end }}"}}")
+           You can watch the status of by running 'kubectl get --namespace {{ .Release.Namespace }} svc -w {{ include "simple-chart.fullname" . }}'
+  export SERVICE_IP=$(kubectl get svc --namespace {{ .Release.Namespace }} {{ include "simple-chart.fullname" . }} --template "{{"{{ range (index .status.loadBalancer.ingress 0) }}{{.}}{{ end }}"}}")
   echo http://$SERVICE_IP:{{ .Values.service.port }}
 {{- else if contains "ClusterIP" .Values.service.type }}
-  export POD_NAME=$(kubectl get pods --namespace {{ .Release.Namespace }} -l "app.kubernetes.io/name={{ include "simple-chart-diff.name" . }},app.kubernetes.io/instance={{ .Release.Name }}" -o jsonpath="{.items[0].metadata.name}")
+  export POD_NAME=$(kubectl get pods --namespace {{ .Release.Namespace }} -l "app.kubernetes.io/name={{ include "simple-chart.name" . }},app.kubernetes.io/instance={{ .Release.Name }}" -o jsonpath="{.items[0].metadata.name}")
   export CONTAINER_PORT=$(kubectl get pod --namespace {{ .Release.Namespace }} $POD_NAME -o jsonpath="{.spec.containers[0].ports[0].containerPort}")
   echo "Visit http://127.0.0.1:8080 to use your application"
   kubectl --namespace {{ .Release.Namespace }} port-forward $POD_NAME 8080:$CONTAINER_PORT
diff --git a/simple-chart-diff/templates/_helpers.tpl b/simple/simple-chart/templates/_helpers.tpl
index 4e025c0..51392be 100644
--- a/simple-chart-diff/templates/_helpers.tpl
+++ b/simple/simple-chart/templates/_helpers.tpl
@@ -1,7 +1,7 @@
 {{/*
 Expand the name of the chart.
 */}}
-{{- define "simple-chart-diff.name" -}}
+{{- define "simple-chart.name" -}}
 {{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
 {{- end }}
 
@@ -10,7 +10,7 @@ Create a default fully qualified app name.
 We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
 If release name contains chart name it will be used as a full name.
 */}}
-{{- define "simple-chart-diff.fullname" -}}
+{{- define "simple-chart.fullname" -}}
 {{- if .Values.fullnameOverride }}
 {{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
 {{- else }}
@@ -26,16 +26,16 @@ If release name contains chart name it will be used as a full name.
 {{/*
 Create chart name and version as used by the chart label.
 */}}
-{{- define "simple-chart-diff.chart" -}}
+{{- define "simple-chart.chart" -}}
 {{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
 {{- end }}
 
 {{/*
 Common labels
 */}}
-{{- define "simple-chart-diff.labels" -}}
-helm.sh/chart: {{ include "simple-chart-diff.chart" . }}
-{{ include "simple-chart-diff.selectorLabels" . }}
+{{- define "simple-chart.labels" -}}
+helm.sh/chart: {{ include "simple-chart.chart" . }}
+{{ include "simple-chart.selectorLabels" . }}
 {{- if .Chart.AppVersion }}
 app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
 {{- end }}
@@ -45,17 +45,17 @@ app.kubernetes.io/managed-by: {{ .Release.Service }}
 {{/*
 Selector labels
 */}}
-{{- define "simple-chart-diff.selectorLabels" -}}
-app.kubernetes.io/name: {{ include "simple-chart-diff.name" . }}
+{{- define "simple-chart.selectorLabels" -}}
+app.kubernetes.io/name: {{ include "simple-chart.name" . }}
 app.kubernetes.io/instance: {{ .Release.Name }}
 {{- end }}
 
 {{/*
 Create the name of the service account to use
 */}}
-{{- define "simple-chart-diff.serviceAccountName" -}}
+{{- define "simple-chart.serviceAccountName" -}}
 {{- if .Values.serviceAccount.create }}
-{{- default (include "simple-chart-diff.fullname" .) .Values.serviceAccount.name }}
+{{- default (include "simple-chart.fullname" .) .Values.serviceAccount.name }}
 {{- else }}
 {{- default "default" .Values.serviceAccount.name }}
 {{- end }}
diff --git a/simple-chart-diff/templates/deployment.yaml b/simple/simple-chart/templates/deployment.yaml
index 42fffe0..1f693a4 100644
--- a/simple-chart-diff/templates/deployment.yaml
+++ b/simple/simple-chart/templates/deployment.yaml
@@ -1,16 +1,16 @@
 apiVersion: apps/v1
 kind: Deployment
 metadata:
-  name: {{ include "simple-chart-diff.fullname" . }}
+  name: {{ include "simple-chart.fullname" . }}
   labels:
-    {{- include "simple-chart-diff.labels" . | nindent 4 }}
+    {{- include "simple-chart.labels" . | nindent 4 }}
 spec:
   {{- if not .Values.autoscaling.enabled }}
   replicas: {{ .Values.replicaCount }}
   {{- end }}
   selector:
     matchLabels:
-      {{- include "simple-chart-diff.selectorLabels" . | nindent 6 }}
+      {{- include "simple-chart.selectorLabels" . | nindent 6 }}
   template:
     metadata:
       {{- with .Values.podAnnotations }}
@@ -18,13 +18,13 @@ spec:
         {{- toYaml . | nindent 8 }}
       {{- end }}
       labels:
-        {{- include "simple-chart-diff.selectorLabels" . | nindent 8 }}
+        {{- include "simple-chart.selectorLabels" . | nindent 8 }}
     spec:
       {{- with .Values.imagePullSecrets }}
       imagePullSecrets:
         {{- toYaml . | nindent 8 }}
       {{- end }}
-      serviceAccountName: {{ include "simple-chart-diff.serviceAccountName" . }}
+      serviceAccountName: {{ include "simple-chart.serviceAccountName" . }}
       securityContext:
         {{- toYaml .Values.podSecurityContext | nindent 8 }}
       containers:
diff --git a/simple-chart-diff/templates/hpa.yaml b/simple/simple-chart/templates/hpa.yaml
index 559b94e..cdc6ba8 100644
--- a/simple-chart-diff/templates/hpa.yaml
+++ b/simple/simple-chart/templates/hpa.yaml
@@ -2,14 +2,14 @@
 apiVersion: autoscaling/v2beta1
 kind: HorizontalPodAutoscaler
 metadata:
-  name: {{ include "simple-chart-diff.fullname" . }}
+  name: {{ include "simple-chart.fullname" . }}
   labels:
-    {{- include "simple-chart-diff.labels" . | nindent 4 }}
+    {{- include "simple-chart.labels" . | nindent 4 }}
 spec:
   scaleTargetRef:
     apiVersion: apps/v1
     kind: Deployment
-    name: {{ include "simple-chart-diff.fullname" . }}
+    name: {{ include "simple-chart.fullname" . }}
   minReplicas: {{ .Values.autoscaling.minReplicas }}
   maxReplicas: {{ .Values.autoscaling.maxReplicas }}
   metrics:
diff --git a/simple-chart-diff/templates/ingress.yaml b/simple/simple-chart/templates/ingress.yaml
index 75b6094..f054e91 100644
--- a/simple-chart-diff/templates/ingress.yaml
+++ b/simple/simple-chart/templates/ingress.yaml
@@ -1,5 +1,5 @@
 {{- if .Values.ingress.enabled -}}
-{{- $fullName := include "simple-chart-diff.fullname" . -}}
+{{- $fullName := include "simple-chart.fullname" . -}}
 {{- $svcPort := .Values.service.port -}}
 {{- if and .Values.ingress.className (not (semverCompare ">=1.18-0" .Capabilities.KubeVersion.GitVersion)) }}
   {{- if not (hasKey .Values.ingress.annotations "kubernetes.io/ingress.class") }}
@@ -17,7 +17,7 @@ kind: Ingress
 metadata:
   name: {{ $fullName }}
   labels:
-    {{- include "simple-chart-diff.labels" . | nindent 4 }}
+    {{- include "simple-chart.labels" . | nindent 4 }}
   {{- with .Values.ingress.annotations }}
   annotations:
     {{- toYaml . | nindent 4 }}
diff --git a/simple-chart-diff/templates/service.yaml b/simple/simple-chart/templates/service.yaml
index 03417eb..f4bb7eb 100644
--- a/simple-chart-diff/templates/service.yaml
+++ b/simple/simple-chart/templates/service.yaml
@@ -1,9 +1,9 @@
 apiVersion: v1
 kind: Service
 metadata:
-  name: {{ include "simple-chart-diff.fullname" . }}
+  name: {{ include "simple-chart.fullname" . }}
   labels:
-    {{- include "simple-chart-diff.labels" . | nindent 4 }}
+    {{- include "simple-chart.labels" . | nindent 4 }}
 spec:
   type: {{ .Values.service.type }}
   ports:
@@ -12,4 +12,4 @@ spec:
       protocol: TCP
       name: http
   selector:
-    {{- include "simple-chart-diff.selectorLabels" . | nindent 4 }}
+    {{- include "simple-chart.selectorLabels" . | nindent 4 }}
diff --git a/simple-chart-diff/templates/serviceaccount.yaml b/simple/simple-chart/templates/serviceaccount.yaml
index ba2c7e1..a17d70a 100644
--- a/simple-chart-diff/templates/serviceaccount.yaml
+++ b/simple/simple-chart/templates/serviceaccount.yaml
@@ -2,9 +2,9 @@
 apiVersion: v1
 kind: ServiceAccount
 metadata:
-  name: {{ include "simple-chart-diff.serviceAccountName" . }}
+  name: {{ include "simple-chart.serviceAccountName" . }}
   labels:
-    {{- include "simple-chart-diff.labels" . | nindent 4 }}
+    {{- include "simple-chart.labels" . | nindent 4 }}
   {{- with .Values.serviceAccount.annotations }}
   annotations:
     {{- toYaml . | nindent 4 }}
diff --git a/simple-chart-diff/templates/tests/test-connection.yaml b/simple/simple-chart/templates/tests/test-connection.yaml
index 70170b3..5733012 100644
--- a/simple-chart-diff/templates/tests/test-connection.yaml
+++ b/simple/simple-chart/templates/tests/test-connection.yaml
@@ -1,9 +1,9 @@
 apiVersion: v1
 kind: Pod
 metadata:
-  name: "{{ include "simple-chart-diff.fullname" . }}-test-connection"
+  name: "{{ include "simple-chart.fullname" . }}-test-connection"
   labels:
-    {{- include "simple-chart-diff.labels" . | nindent 4 }}
+    {{- include "simple-chart.labels" . | nindent 4 }}
   annotations:
     "helm.sh/hook": test
 spec:
@@ -11,5 +11,5 @@ spec:
     - name: wget
       image: busybox
       command: ['wget']
-      args: ['{{ include "simple-chart-diff.fullname" . }}:{{ .Values.service.port }}']
+      args: ['{{ include "simple-chart.fullname" . }}:{{ .Values.service.port }}']
   restartPolicy: Never
diff --git a/simple-chart-diff/values.yaml b/simple/simple-chart/values.yaml
index 704dd85..e95f0c7 100644
--- a/simple-chart-diff/values.yaml
+++ b/simple/simple-chart/values.yaml
@@ -1,12 +1,14 @@
-# Default values for simple-chart-diff.
+# Default values for simple-chart.
 # This is a YAML-formatted file.
 # Declare variables to be passed into your templates.
 
 replicaCount: 1
 
 image:
-  repository: nginx
-  pullPolicy: IfNotPresent
+  repository: simple-webservice
+  # IfNotPresent can be used here, but I dont want to upload the docker images
+  # anywhere
+  pullPolicy: Never
   # Overrides the image tag whose default is the chart appVersion.
   tag: ""
 
@@ -38,7 +40,7 @@ securityContext: {}
 
 service:
   type: ClusterIP
-  port: 80
+  port: 8080
 
 ingress:
   enabled: false
```
