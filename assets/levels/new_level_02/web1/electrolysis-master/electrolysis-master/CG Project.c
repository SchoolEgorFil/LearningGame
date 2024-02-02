#include<stdio.h>
#include<GL/glut.h>
#include<math.h>
#include<time.h>

#define ARRAY_SIZE(array) (sizeof(array)/sizeof(array[0]))

struct wcPt3D
{
    GLfloat x, y, z;
};

clock_t start_time;
int i,j,choice,ionsReached,limit2=3,animate,createBubbles,k=0;;
int limit1[4]={3,3,3,3};
GLint xRaster,yRaster,xTrans,yTrans,zTrans,nCtrlPts=4,nCtrlPts_2=4,nBezCurvePts,nBezCurvePts_2;
GLint ionIndex[4]={0,0,0,0};
GLint ionIndex_2[4]={0,0,0,0};
GLint C[4];
GLint C_2[4];
//Control points for Na+ ions
struct wcPt3D ctrlPts [4][4] =   { { {605, 280.0, 0.0}, {675.0, 326.0, 0.0}, {745.0, 373.0, 0.0}, {815.0, 420.0, 0.0} },
                                 { {600, 245.0, 0.0}, {670.0, 295.0, 0.0}, {740.0, 337.0, 0.0}, {815.0, 380.0, 0.0} },
                                 { {690, 280.0, 0.0}, {733.0, 300.0, 0.0}, {777.0, 320.0, 0.0}, {820.0, 340.0, 0.0} },
                                 { {655, 225.0, 0.0}, {723.0, 263.0, 0.0}, {792.0, 302.0, 0.0}, {860.0, 340.0, 0.0} } };
//Control points for Cl- ions
struct wcPt3D ctrlPts_2 [4][4] = { { {675, 255.0, 0.0}, {562.0, 340.0, 0.0}, {499.0, 380.0, 0.0}, {435.0, 420.0, 0.0} },
                                 { {645, 275.0, 0.0}, {562.0, 317.0, 0.0}, {499.0, 370.0, 0.0}, {435.0, 380.0, 0.0} },
                                 { {625, 245.0, 0.0}, {562.0, 280.0, 0.0}, {499.0, 317.0, 0.0}, {430.0, 340.0, 0.0} },
                                 { {595, 225.0, 0.0}, {513.0, 263.0, 0.0}, {453.0, 301.0, 0.0}, {390.0, 340.0, 0.0} } };
GLfloat u,u_2;
float xc_1[4]={605,600,690,655};
float yc_1[4]={280,245,280,225};
float xc_2[4]={675,645,625,595};
float yc_2[4]={255,275,245,225};
float xc_3[4]={400, 250, 706, 1000};
float yc_3[4]={530, 218, 125, 218};
float xc_4[4]={435, 435, 430, 390};
float yc_4[4]={420, 380, 340, 340};

float bubblesCoords_x[4]={435.0,435.0,430.0,390.0};
float bubblesCoords_y[4]={420.0,380.0,340.0,340.0};

struct wcPt3D bezCurvePt;
struct wcPt3D bezCurvePt_2;

void draw_ions_1();
void draw_ions_2();
void calculateElectronsCoords();
void calculateElectronsCoords_2();
void plotPoint (struct wcPt3D bezCurvePt);
void binomialCoeffs (GLint n, GLint *C);
void computeBezPt (GLfloat u, struct wcPt3D *bezPt, GLint nCtrlPts, struct wcPt3D *ctrlPts, GLint *C);

void display()
{
	glClear(GL_COLOR_BUFFER_BIT);
	if(choice==0)
    {
        glLineWidth(6);
        glColor3f(0.29,0.0,0.507);
        xRaster=900;
        yRaster=100;
        glRasterPos2i(xRaster,yRaster);
        char name1[]="Harry Roshan (1PE15CS056)";
        char name2[]="Chethan Reddy T R (1PE15CS044)";
        for(i=0;i<ARRAY_SIZE(name1);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,name1[i]);
        }
        yRaster-=30;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(name2);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,name2[i]);
        }
        glBlendFunc(GL_SRC_ALPHA,GL_ONE_MINUS_SRC_ALPHA);
        glEnable(GL_BLEND);
        glEnable(GL_LINE_SMOOTH);
        glPushMatrix();
        xTrans=220;
        yTrans=550;
        zTrans=0;
        glTranslatef(xTrans,yTrans,zTrans);
        glScalef(0.7,0.7,1);
        char title1[]="VISUALIZATION OF";
        char title2[]="ELECTROLYSIS";
        for(i=0;i<ARRAY_SIZE(title1);i++)
        {
            glutStrokeCharacter(GLUT_STROKE_ROMAN,title1[i]);
        }
        glPopMatrix();
        glPushMatrix();
        yTrans-=100;
        xTrans+=90;
        glTranslatef(xTrans,yTrans,zTrans);
        glScalef(0.7,0.7,1);
        for(i=0;i<ARRAY_SIZE(title2);i++)
        {
            glutStrokeCharacter(GLUT_STROKE_ROMAN,title2[i]);
        }
        glPopMatrix();

        glDisable(GL_BLEND);
        glDisable(GL_LINE_SMOOTH);

        glBlendFunc(GL_SRC_ALPHA,GL_ONE_MINUS_SRC_ALPHA);
        glEnable(GL_BLEND);
        glEnable(GL_LINE_SMOOTH);

        char msg1[]="Press 1 To View The Visualization of Electrolysis";
        char msg2[]="Press 2 To Read About Electrolysis";
        char msg3[]="Press 3 To Exit";
        glLineWidth(2.25);
        glPushMatrix();
        yTrans=150;
        xTrans=10;
        glTranslatef(xTrans,yTrans,zTrans);
        glScalef(0.2,0.2,1);
        for(i=0;i<ARRAY_SIZE(msg1);i++)
        {
            glutStrokeCharacter(GLUT_STROKE_ROMAN,msg1[i]);
        }
        glPopMatrix();

        glPushMatrix();
        yTrans-=40;
        glTranslatef(xTrans,yTrans,zTrans);
        glScalef(0.2,0.2,1);
        for(i=0;i<ARRAY_SIZE(msg2);i++)
        {
            glutStrokeCharacter(GLUT_STROKE_ROMAN,msg2[i]);
        }
        glPopMatrix();

        glPushMatrix();
        yTrans-=40;;
        glTranslatef(xTrans,yTrans,zTrans);
        glScalef(0.2,0.2,1);
        for(i=0;i<ARRAY_SIZE(msg3);i++)
        {
            glutStrokeCharacter(GLUT_STROKE_ROMAN,msg3[i]);
        }
        glPopMatrix();

        glDisable(GL_BLEND);
        glDisable(GL_LINE_SMOOTH);
        glFlush();
    }

	else if(choice==1)
    {
        glClearColor(1.0,1.0,1.0,1.0);
        glColor3f(0.3,0.3,1.0);
        //glBlendFunc(GL_SRC_ALPHA,GL_ONE_MINUS_SRC_ALPHA);

        glBegin(GL_POLYGON);
            glVertex2f(325.0,200.0);
            glVertex2f(325.0,480.0);
            glVertex2f(925.0,480.0);
            glVertex2f(925.0,200.0);
        glEnd();

        glColor3f(0.644,0.164,0.164);
        glBegin(GL_POLYGON);
            glVertex2f(375.0,350.0);
            glVertex2f(375.0,530.0);
            glVertex2f(425.0,530.0);
            glVertex2f(425.0,350.0);
        glEnd();

        glBegin(GL_POLYGON);
            glVertex2f(875.0,350.0);
            glVertex2f(875.0,530.0);
            glVertex2f(825.0,530.0);
            glVertex2f(825.0,350.0);
        glEnd();

        /*Beaker Coords*/
        glColor3f(0.0,0.0,0.0);
        glBegin(GL_LINES);
            glVertex2f(325.0,500.0);
            glVertex2f(325.0,200.0);
            glVertex2f(325.0,200.0);
            glVertex2f(925.0,200.0);
            glVertex2f(925.0,200.0);
            glVertex2f(925.0,500.0);
        glEnd();

        glBegin(GL_LINE_LOOP);
            glVertex2f(375.0,350.0);
            glVertex2f(375.0,530.0);
            glVertex2f(425.0,530.0);
            glVertex2f(425.0,350.0);
        glEnd();

        glBegin(GL_LINE_LOOP);
            glVertex2f(875.0,350.0);
            glVertex2f(875.0,530.0);
            glVertex2f(825.0,530.0);
            glVertex2f(825.0,350.0);
        glEnd();

        glLineWidth(4.25);
        glBegin(GL_LINES);
            glVertex2f(400,530);
            glVertex2f(400,570);
            glVertex2f(400,570);
            glVertex2f(250,570);
            glVertex2f(250,570);
            glVertex2f(250,125);
            glVertex2f(250,125);
            glVertex2f(1025,125);
            glVertex2f(1025,125);
            glVertex2f(1025,110);
            glVertex2f(850,530);
            glVertex2f(850,570);
            glVertex2f(850,570);
            glVertex2f(1000,570);
            glVertex2f(1000,570);
            glVertex2f(1000,160);
            glVertex2f(1000,160);
            glVertex2f(1105,160);
            glVertex2f(1105,160);
            glVertex2f(1105,110);
        glEnd();

        glLineWidth(2.20);
        glColor3f(0.25,0.25,0.25);
        glBegin(GL_POLYGON);
            glVertex2f(1000.0,20.0);
            glVertex2f(1000.0,100.0);
            glVertex2f(1130.0,100.0);
            glVertex2f(1130.0,20.0);
        glEnd();

        glColor3f(0.0,0.0,0.3);
        glBegin(GL_POLYGON);
            glVertex2f(1010.0,100.0);
            glVertex2f(1010.0,110.0);
            glVertex2f(1040.0,110.0);
            glVertex2f(1040.0,100.0);
        glEnd();

        glColor3f(1.0,0.0,0.0);
        glBegin(GL_POLYGON);
            glVertex2f(1090.0,100.0);
            glVertex2f(1090.0,110.0);
            glVertex2f(1120.0,110.0);
            glVertex2f(1120.0,100.0);
        glEnd();

        glColor3f(0.0,0.0,0.0);
        glBegin(GL_LINE_LOOP);
            glVertex2f(1000.0,20.0);
            glVertex2f(1000.0,100.0);
            glVertex2f(1130.0,100.0);
            glVertex2f(1130.0,20.0);
        glEnd();

        glBegin(GL_LINE_LOOP);
            glVertex2f(1010.0,100.0);
            glVertex2f(1010.0,110.0);
            glVertex2f(1040.0,110.0);
            glVertex2f(1040.0,100.0);
        glEnd();

        glBegin(GL_LINE_LOOP);
            glVertex2f(1090.0,100.0);
            glVertex2f(1090.0,110.0);
            glVertex2f(1120.0,110.0);
            glVertex2f(1120.0,100.0);
        glEnd();

        glColor3f(0.0,0.0,0.0);

        xRaster=845;
        yRaster=478;
        glRasterPos2i(xRaster,yRaster);
        char name1[]="CATHODE";
        for(i=0;i<ARRAY_SIZE(name1);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_HELVETICA_12,name1[i]);
            yRaster-=15;
            glRasterPos2i(xRaster,yRaster);
        }

        xRaster=395;
        yRaster=470;
        glRasterPos2i(xRaster,yRaster);
        char name2[]="ANODE";
        for(i=0;i<ARRAY_SIZE(name1);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_HELVETICA_12,name2[i]);
            yRaster-=15;
            glRasterPos2i(xRaster,yRaster);
        }

        xRaster=1030;
        yRaster=60;
        glRasterPos2i(xRaster,yRaster);
        char name3[]="BATTERY";
        for(i=0;i<ARRAY_SIZE(name3);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_HELVETICA_12,name3[i]);
            xRaster+=10;
            glRasterPos2i(xRaster,yRaster);
        }

        glColor3f(1.0,1.0,1.0);
        xRaster=1103;
        yRaster=102;
        glRasterPos2i(xRaster,yRaster);
        char name4='+';
        glutBitmapCharacter(GLUT_BITMAP_HELVETICA_12,name4);
        glRasterPos2i(xRaster,yRaster);

        xRaster=1023;
        yRaster=102;
        glRasterPos2i(xRaster,yRaster);
        char name5='-';
        glutBitmapCharacter(GLUT_BITMAP_HELVETICA_12,name5);
        glRasterPos2i(xRaster,yRaster);

        glMatrixMode (GL_MODELVIEW);
        //To draw circle...

        if(animate==1 && clock()>(start_time+250000))
            draw_ions_1();
        for(j=0;j<4;j++)
        {
            glColor3f(0.29,0.0,0.507);
            GLUquadricObj *circle;
            glPushMatrix();
            glTranslatef (xc_1[j], yc_1[j], 0.0);
            circle=gluNewQuadric();
            gluQuadricDrawStyle (circle, GLU_FILL);
            gluDisk (circle, 0,10,50,1);
            glPopMatrix();

            glColor3f(1.0,1.0,1.0);
            xRaster=xc_1[j]-9;
            yRaster=yc_1[j]-3;
            glRasterPos2i(xRaster,yRaster);
            char ions[100]="Na+";
            for(i=0;i<limit1[j];i++)
            {
                glutBitmapCharacter(GLUT_BITMAP_HELVETICA_10,ions[i]);
            }
        }

        if(animate==1 && clock()>(start_time+250000))
            draw_ions_2();
        if(createBubbles==0  || clock()<=(start_time+1500000))
        {
            for(j=0;j<4;j++)
            {
                glColor3f(0.0,0.7,0.0);
                GLUquadricObj *circle2;
                glPushMatrix();
                glTranslatef (xc_2[j], yc_2[j], 0.0);
                circle2=gluNewQuadric();
                gluQuadricDrawStyle (circle2, GLU_FILL);
                gluDisk (circle2, 0,10,50,1);
                glPopMatrix();

                glColor3f(1.0,1.0,1.0);
                xRaster=xc_2[j]-9;
                yRaster=yc_2[j]-3;
                glRasterPos2i(xRaster,yRaster);
                char ions2[100]="Cl-";
                for(i=0;i<limit2;i++)
                {
                    glutBitmapCharacter(GLUT_BITMAP_HELVETICA_10,ions2[i]);
                }
            }
        }

        for(j=0;j<4;j++)
        {
            glColor3f(0.125,0.6953125,0.6640625);
            GLUquadricObj *circle3;
            glPushMatrix();
            glTranslatef (xc_3[j], yc_3[j], 0.0);
            circle3=gluNewQuadric();
            gluQuadricDrawStyle (circle3, GLU_FILL);
            gluDisk (circle3, 0,7,50,1);
            glPopMatrix();

            glColor3f(1.0,1.0,1.0);
            xRaster=xc_3[j]-6;
            yRaster=yc_3[j]-2;
            glRasterPos2i(xRaster,yRaster);
            char electron[100]="e-";
            for(i=0;i<2;i++)
            {
                glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_10,electron[i]);
            }
        }

        ionsReached=0;
        createBubbles=0;

        if(clock()>(start_time+800000))
        {
            //Drawing electrons from Chlorine ions
            if(xc_4[0]!=815 || yc_4[0]!=420)
            {
                j=0;
                if(ionIndex[j]>nBezCurvePts)
                {
                    glColor3f(0.125,0.6953125,0.6640625);
                    GLUquadricObj *circle3;
                    glPushMatrix();
                    glTranslatef (xc_4[j], yc_4[j], 0.0);
                    circle3=gluNewQuadric();
                    gluQuadricDrawStyle (circle3, GLU_FILL);
                    gluDisk (circle3, 0,7,50,1);
                    glPopMatrix();

                    glColor3f(1.0,1.0,1.0);
                    xRaster=xc_4[j]-6;
                    yRaster=yc_4[j]-2;
                    glRasterPos2i(xRaster,yRaster);
                    char electron[100]="e-";
                    for(i=0;i<2;i++)
                    {
                        glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_10,electron[i]);
                    }
                    ionsReached=1;
                }
            }
            else
                limit1[0]=2;
            if(xc_4[1]!=814 || yc_4[1]!=380)
            {
                j=1;
                if(ionIndex[j]>nBezCurvePts)
                {
                    glColor3f(0.125,0.6953125,0.6640625);
                    GLUquadricObj *circle3;
                    glPushMatrix();
                    glTranslatef (xc_4[j], yc_4[j], 0.0);
                    circle3=gluNewQuadric();
                    gluQuadricDrawStyle (circle3, GLU_FILL);
                    gluDisk (circle3, 0,7,50,1);
                    glPopMatrix();

                    glColor3f(1.0,1.0,1.0);
                    xRaster=xc_4[j]-6;
                    yRaster=yc_4[j]-2;
                    glRasterPos2i(xRaster,yRaster);
                    char electron[100]="e-";
                    for(i=0;i<2;i++)
                    {
                        glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_10,electron[i]);
                    }
                    ionsReached=1;
                }
            }
            else
                limit1[1]=2;
            if(xc_4[2]!=819 || yc_4[2]!=340)
            {
                j=2;
                if(ionIndex[j]>nBezCurvePts)
                {
                    glColor3f(0.125,0.6953125,0.6640625);
                    GLUquadricObj *circle3;
                    glPushMatrix();
                    glTranslatef (xc_4[j], yc_4[j], 0.0);
                    circle3=gluNewQuadric();
                    gluQuadricDrawStyle (circle3, GLU_FILL);
                    gluDisk (circle3, 0,7,50,1);
                    glPopMatrix();

                    glColor3f(1.0,1.0,1.0);
                    xRaster=xc_4[j]-6;
                    yRaster=yc_4[j]-2;
                    glRasterPos2i(xRaster,yRaster);
                    char electron[100]="e-";
                    for(i=0;i<2;i++)
                    {
                        glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_10,electron[i]);
                    }
                    ionsReached=1;
                }
            }
            else
                limit1[2]=2;
            if(xc_4[3]!=860 || yc_4[3]!=340)
            {
                j=3;
                if(ionIndex[j]>nBezCurvePts)
                {
                    glColor3f(0.125,0.6953125,0.6640625);
                    GLUquadricObj *circle3;
                    glPushMatrix();
                    glTranslatef (xc_4[j], yc_4[j], 0.0);
                    circle3=gluNewQuadric();
                    gluQuadricDrawStyle (circle3, GLU_FILL);
                    gluDisk (circle3, 0,7,50,1);
                    glPopMatrix();

                    glColor3f(1.0,1.0,1.0);
                    xRaster=xc_4[j]-6;
                    yRaster=yc_4[j]-2;
                    glRasterPos2i(xRaster,yRaster);
                    char electron[100]="e-";
                    for(i=0;i<2;i++)
                    {
                        glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_10,electron[i]);
                    }
                    ionsReached=1;
                }
            }
            else
            {
                limit1[3]=2;
                createBubbles=1;
            }
        }

        if(ionsReached==1 && animate==1)
        {
            calculateElectronsCoords_2();
        }
        if(xc_2[0]==435 && yc_2[0]==420 && clock()>(start_time+800000))
        {
            limit2=2;
        }

        if(createBubbles==1 && clock()>(start_time+1500000) && animate==1)
        {
            for(i=0;i<4;i++)
            {
                glColor3f(0,0,0);
                GLUquadricObj *circle4;
                glPushMatrix();
                glTranslatef (bubblesCoords_x[i]-10, bubblesCoords_y[i], 0.0);
                circle4=gluNewQuadric();
                gluQuadricDrawStyle (circle4, GLU_FILL);
                gluDisk (circle4, 1,2,50,1);
                glPopMatrix();

                glColor3f(0,0,0);
                glPushMatrix();
                glTranslatef (bubblesCoords_x[i], bubblesCoords_y[i], 0.0);
                circle4=gluNewQuadric();
                gluQuadricDrawStyle (circle4, GLU_FILL);
                gluDisk (circle4, 1,2,50,1);
                glPopMatrix();

                glColor3f(0,0,0);
                glPushMatrix();
                glTranslatef (bubblesCoords_x[i]+10, bubblesCoords_y[i], 0.0);
                circle4=gluNewQuadric();
                gluQuadricDrawStyle (circle4, GLU_FILL);
                gluDisk (circle4, 1,2,50,1);
                glPopMatrix();

                glColor3f(0,0,0);
                glPushMatrix();
                glTranslatef (bubblesCoords_x[i]-8, bubblesCoords_y[i]+10, 0.0);
                circle4=gluNewQuadric();
                gluQuadricDrawStyle (circle4, GLU_FILL);
                gluDisk (circle4, 1,2,50,1);
                glPopMatrix();

                glColor3f(0,0,0);
                glPushMatrix();
                glTranslatef (bubblesCoords_x[i]+8, bubblesCoords_y[i]+10, 0.0);
                circle4=gluNewQuadric();
                gluQuadricDrawStyle (circle4, GLU_FILL);
                gluDisk (circle4, 1,2,50,1);
                glPopMatrix();

                glColor3f(0,0,0);
                glPushMatrix();
                glTranslatef (bubblesCoords_x[i]-8, bubblesCoords_y[i]-10, 0.0);
                circle4=gluNewQuadric();
                gluQuadricDrawStyle (circle4, GLU_FILL);
                gluDisk (circle4, 1,2,50,1);
                glPopMatrix();

                glColor3f(0,0,0);
                glPushMatrix();
                glTranslatef (bubblesCoords_x[i]+8, bubblesCoords_y[i]-10, 0.0);
                circle4=gluNewQuadric();
                gluQuadricDrawStyle (circle4, GLU_FILL);
                gluDisk (circle4, 1,2,50,1);
                glPopMatrix();

                bubblesCoords_y[i]+=0.5;
            }
        }

        nCtrlPts = 4, nBezCurvePts = 300;
        nBezCurvePts_2 = 300;
        nCtrlPts_2=4;
        if(animate==1)
            calculateElectronsCoords();

        glFlush();
        glutPostRedisplay();
        glutSwapBuffers();
    }
    else if(choice==2)
    {
        glClearColor(1.0,1.0,1.0,1.0);
        glColor3f(0.0,0.0,0.0);
        xRaster=75;
        yRaster=650;
        glRasterPos2i(xRaster,yRaster);
        char desc[]="In chemistry and manufacturing, electrolysis is a technique that uses a direct electric current(DC) to drive an ";
        char desc1[]="otherwise non-spontaneous chemical reaction. It is the passing of a direct electric current through an ionic ";
        char desc2[]="substance that is either molten or dissolved in a suitable solvent, producing chemical reactions at the electrodes";
        char desc3[]="and separation of materials. Electrolysis is commercially important as a stage in the separation  of elements from";
        char desc4[]="naturally occurring sources such as ores using an electrolytic cell. This project will provide a visualization of";
        char desc5[]="the electrolysis process which can increase the understanding of the topic for students, teachers, workers etc.";
        char desc6[]="This visualization can serve as a supplement to text book learning.";
        char desc7[]="In our visualization, we show the electrolysis of molten NaCl:-";
        char desc8[]="Cathode and Anode    -    Copper";
        char desc9[]="Electrolyte                   -    Molten NaCl";
        char desc10[]="Battery                         -    5V";
        for(i=0;i<ARRAY_SIZE(desc);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc[i]);
        }
        xRaster=75;
        yRaster=600;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc1);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc1[i]);
        }
        xRaster=75;
        yRaster=550;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc2);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc2[i]);
        }
        xRaster=75;
        yRaster=500;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc3);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc3[i]);
        }
        xRaster=75;
        yRaster=450;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc4);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc4[i]);
        }
        xRaster=75;
        yRaster=400;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc5);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc5[i]);
        }
        xRaster=75;
        yRaster=350;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc6);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc6[i]);
        }
        xRaster=75;
        yRaster=250;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc7);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc7[i]);
        }
        xRaster=75;
        yRaster=200;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc8);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc8[i]);
        }
        xRaster=75;
        yRaster=150;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc9);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc9[i]);
        }
        xRaster=75;
        yRaster=100;
        glRasterPos2i(xRaster,yRaster);
        for(i=0;i<ARRAY_SIZE(desc10);i++)
        {
            glutBitmapCharacter(GLUT_BITMAP_TIMES_ROMAN_24,desc10[i]);
        }
        glFlush();
    }
    else if(choice==3)
    {
        exit(0);
    }
    glutSwapBuffers();
}

void keys(unsigned char key, int x, int y)
{
    if(key=='1')
    {
        printf("Choice=1\n");
        choice=1;
    }
    if(key=='2')
    {
        printf("Choice=2\n");
        choice=2;
    }
    if(key=='3')
    {
        printf("Exiting\n");
        choice=3;
    }
    if(key==27)
    {
        printf("Display Main Menu\n");
        choice=0;
        animate=0;
    }
    display();
}

void draw_ions_1()
{
    /* Set example number of control points and number of
    * curve positions to be plotted along the Bezier curve.
    */
    binomialCoeffs (nCtrlPts - 1, C);
    int y;
    for(y=0;y<4;y++)
    {
        if(ionIndex[y] <= nBezCurvePts)
        {
            u = (GLfloat) ionIndex[y] / (GLfloat) nBezCurvePts;
            computeBezPt (u, &bezCurvePt, nCtrlPts, ctrlPts[y], C);
            xc_1[y]=bezCurvePt.x;
            yc_1[y]=bezCurvePt.y;
            ionIndex[y]++;
        }
    }
    //glutPostRedisplay();
    if(animate==1)
        glutTimerFunc(1500,draw_ions_1,0);
}

void draw_ions_2()
{
    binomialCoeffs (nCtrlPts_2 - 1, C_2);
    int y;
    for(y=0;y<4;y++)
    {
        if(ionIndex_2[y] <= nBezCurvePts_2)
        {
            u_2 = (GLfloat) ionIndex_2[y] / (GLfloat) nBezCurvePts_2;
            computeBezPt (u_2, &bezCurvePt_2, nCtrlPts, ctrlPts_2[y], C_2);
            xc_2[y]=bezCurvePt_2.x;
            yc_2[y]=bezCurvePt_2.y;
            ionIndex_2[y]++;
        }
    }
    //glutPostRedisplay();
    if(animate==1)
        glutTimerFunc(1500,draw_ions_2,0);
}

void calculateElectronsCoords()
{
    int z;
    for(z=0;z<4;z++)
    {
        if(xc_3[z]==850 && yc_3[z]==530)
            xc_3[z]=400;
        if(xc_3[z]==400 && yc_3[z]<570 && yc_3[z]>=530)
        {
            yc_3[z]+=3;
        }
        else if(xc_3[z]<=400 && xc_3[z]>250 && yc_3[z]==572)
        {
            xc_3[z]-=3;
        }
        else if(xc_3[z]==250 && yc_3[z]<=572 && yc_3[z]>125)
        {
            yc_3[z]-=3;
        }
        else if(xc_3[z]>=250 && xc_3[z]<1025 && yc_3[z]==125)
        {
            xc_3[z]+=3;
        }
        else if(xc_3[z]==1027 && yc_3[z]<=125 && yc_3[z]>110)
        {
            yc_3[z]-=3;
        }
        else if((xc_3[z]==1027 || xc_3[z]==1105) && yc_3[z]>=110 && yc_3[z]<160)
        {
            xc_3[z]=1105;
            yc_3[z]+=3;
        }
        else if(xc_3[z]<=1105 && xc_3[z]>1000 && yc_3[z]==161)
        {
            xc_3[z]-=3;
        }
        else if(xc_3[z]==1000 && yc_3[z]>=161 && yc_3[z]<572)
        {
            yc_3[z]+=3;
        }
        else if(xc_3[z]<=1000  && xc_3[z]>850 && yc_3[z]==572)
        {
            xc_3[z]-=3;
        }
        else if(xc_3[z]==850 && yc_3[z]<=572 && yc_3[z]>530)
        {
            yc_3[z]-=3;
        }
    }
}

void calculateElectronsCoords_2()
{
    int z;
    //1st electron
    if(xc_4[0]>=418 && xc_4[0]<=435)
        xc_4[0]-=3;
    if(yc_4[0]>530)
    {
        xc_4[0]=835;
        yc_4[0]=530;
    }
    if(yc_4[0]<=530 && xc_4[0]>=815 && yc_4[0]>420)
        yc_4[0]-=2;
    if(yc_4[0]==420 && xc_4[0]>815)
        xc_4[0]-=2;

    //2nd electron
    if(xc_4[1]>411 && xc_4[1]<=435)
        xc_4[1]-=3;
    if(yc_4[1]>530)
    {
        xc_4[1]=840;
        yc_4[1]=530;
    }
    if(yc_4[1]<=530 && xc_4[1]>=815 && yc_4[1]>380)
    {
        yc_4[1]-=2;
    }
    if(yc_4[1]==380 && xc_4[1]>815)
    {
        xc_4[1]-=2;
    }

    //3rd electron
    if(xc_4[2]>405 && xc_4[2]<=435)
        xc_4[2]-=3;
    if(yc_4[2]>530)
    {
        xc_4[2]=845;
        yc_4[2]=530;
    }
    if(yc_4[2]<=530 && xc_4[2]>821 && yc_4[2]>340)
    {
        yc_4[2]-=2;
    }
    if(yc_4[2]==340 && xc_4[2]>=820)
    {
        xc_4[2]-=2;
    }

    //4th electron
    if(yc_4[3]>530)
    {
        xc_4[3]=860;
        yc_4[3]=530;
    }
    if(yc_4[3]<=530 && xc_4[3]>=860 && yc_4[3]>340)
    {
        yc_4[3]-=2;
    }

    //Increment all electrons
    for(z=0;z<4;z++)
    {
        if(yc_4[z]<=530 && xc_4[z]<=435)
            yc_4[z]+=2;
    }
}

void plotPoint (struct wcPt3D bezCurvePt)
{
    glBegin (GL_POINTS);
        glVertex2f (bezCurvePt.x, bezCurvePt.y);
    glEnd ( );
}

/* Compute binomial coefficients C for given value of n.*/
void binomialCoeffs (GLint n, GLint *C)
{
    GLint k, j;
    for (k = 0; k <= n; k++)
    {
        /* Compute n!/(k!(n - k)!).*/
        C [k] = 1;
        for (j = n; j >= k + 1;j--)
            C [k] *= j;
        for (j = n - k; j >= 2;j--)
            C [k] /= j;
    }
}

void computeBezPt (GLfloat u, struct wcPt3D *bezPt, GLint nCtrlPts, struct wcPt3D *ctrlPts, GLint *C)
{
    GLint k, n = nCtrlPts - 1;
    GLfloat bezBlendFcn;
    bezPt->x = bezPt->y = bezPt->z = 0.0;
    /* Compute blending functions and blend control points. */
    for (k = 0; k < nCtrlPts; k++)
    {
        bezBlendFcn = C [k] * pow (u, k) * pow (1 - u, n - k);
        bezPt->x += ctrlPts [k].x * bezBlendFcn;
        bezPt->y += ctrlPts [k].y * bezBlendFcn;
        bezPt->z += ctrlPts [k].z * bezBlendFcn;
    }
}

void winReshapeFcn (GLint newWidth, GLint newHeight)
{
    /* Maintain an aspect ratio of 1.0. */
    glViewport (0, 0, newHeight, newHeight);
    glMatrixMode (GL_PROJECTION);
    glLoadIdentity ( );
    gluOrtho2D (0, 1280, 0, 720);
    glClear (GL_COLOR_BUFFER_BIT);
}

void myinit()
{
	glClearColor(1.0,1.0,1.0,1.0);
	glPointSize(1.1);
	glMatrixMode(GL_PROJECTION);
	gluOrtho2D(0,1280,0,720);
}

void menu_func(int id)
{
    switch(id)
    {
        case 1: animate=1;
                start_time=clock();

                xc_1[0]=605;
                xc_1[1]=600;
                xc_1[2]=690;
                xc_1[3]=655;

                yc_1[0]=280;
                yc_1[1]=245;
                yc_1[2]=280;
                yc_1[3]=225;

                xc_2[0]=675;
                xc_2[1]=645;
                xc_2[2]=625;
                xc_2[3]=595;

                yc_2[0]=255;
                yc_2[1]=275;
                yc_2[2]=245;
                yc_2[3]=225;

                xc_3[0]=400;
                xc_3[1]=250;
                xc_3[2]=706;
                xc_3[3]=1000;

                yc_3[0]=530;
                yc_3[1]=218;
                yc_3[2]=125;
                yc_3[3]=218;

                xc_4[0]=435;
                xc_4[1]=435;
                xc_4[2]=430;
                xc_4[3]=390;

                yc_4[0]=420;
                yc_4[1]=380;
                yc_4[2]=340;
                yc_4[3]=340;

                ionIndex[0]=ionIndex[1]=ionIndex[2]=ionIndex[3]=0;
                ionIndex_2[0]=ionIndex_2[1]=ionIndex_2[2]=ionIndex_2[3]=0;

                for(i=0;i<4;i++)
                    limit1[i]=3;
                limit2=3;

                bubblesCoords_x[0]=435.0;
                bubblesCoords_x[1]=435.0;
                bubblesCoords_x[2]=430.0;
                bubblesCoords_x[3]=390.0;

                bubblesCoords_y[0]=420.0;
                bubblesCoords_y[1]=380.0;
                bubblesCoords_y[2]=340.0;
                bubblesCoords_y[3]=340.0;

                display();
                break;
        case 2: animate=0;
                display();
                break;
    }
    glutPostRedisplay();
}

void main(int argc, char **argv)
{
    choice=0;
	glutInit(&argc,argv);
	glutInitDisplayMode(GLUT_DOUBLE|GLUT_RGB);
	glutInitWindowSize(1280,720);
	glutInitWindowPosition(0,0);
	glutCreateWindow("Visualization of Electrolysis");

    glutCreateMenu(menu_func);
    glutAddMenuEntry("Start Animation",1);
    glutAddMenuEntry("Stop Animation",2);
    glutAttachMenu(GLUT_RIGHT_BUTTON);
    animate=0;

	glutDisplayFunc(display);
	glutKeyboardFunc(keys);
	myinit();
	glutMainLoop();
}